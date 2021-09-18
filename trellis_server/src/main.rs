#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use anyhow;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::figment::providers::Env;
use rocket::figment::Figment;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::{status, Redirect};
use rocket::serde::json::Json;
use rocket::State;
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};
use serde_json::map::Map;
use serde_json::Value::{self, Object};
use trellis_core::config;

mod auth;
pub mod models;
pub mod schema;

#[database("trellis")]
struct DbConn(PgConnection);

async fn load_settings(db: DbConn, uid: String) -> anyhow::Result<Option<config::Config>> {
    use schema::settings::dsl::*;
    let res = db
        .run(move |c| {
            settings
                .filter(user_id.eq(uid))
                .first::<models::Settings>(c)
        })
        .await
        .optional()?;

    match res {
        Some(row) => Ok(Some(serde_json::from_value::<config::Config>(row.data)?)),
        None => Ok(Some(config::Config::default())),
    }
}

#[get("/load")]
async fn load(
    db: DbConn,
    cookies: &CookieJar<'_>,
) -> Result<Option<Json<config::Config>>, status::Custom<&'static str>> {
    let user_id = match cookies.get_private("session") {
        None => return Err(status::Custom(Status::Unauthorized, "Unauthorized")),
        Some(cookie) => String::from(cookie.value()),
    };

    match load_settings(db, user_id).await {
        Ok(Some(settings)) => Ok(Some(Json(settings))),
        Ok(None) => Ok(None),
        Err(err) => {
            log::error!("{}", err);
            Err(status::Custom(
                Status::InternalServerError,
                "Internal Server Error",
            ))
        }
    }
}

#[post("/save", data = "<data>")]
async fn save(
    db: DbConn,
    cookies: &CookieJar<'_>,
    data: Json<Value>,
) -> Result<Json<Value>, status::Unauthorized<&'static str>> {
    use schema::settings::dsl;

    let uid = match cookies.get_private("session") {
        None => return Err(status::Unauthorized(Some("Unauthorized"))),
        Some(cookie) => String::from(cookie.value()),
    };

    let new_settings = models::NewSettings {
        data: data.into_inner(),
        user_id: uid,
    };

    let res = db
        .run(move |c| {
            diesel::insert_into(schema::settings::table)
                .values(&new_settings)
                .on_conflict(dsl::user_id)
                .do_update()
                .set(dsl::data.eq(&new_settings.data))
                .execute(c)
        })
        .await;

    match res {
        Ok(_) => Ok(Json(Object(Map::new()))),
        Err(err) => {
            log::error!("{}", err);
            return Err(status::Unauthorized(Some("Unauthorized")));
        }
    }
}

/// Handles HEAD requests as a no-op. This prevents some link sanitizers from consuming the magic
/// link token. Without this, Rocket would otherwise run the equivalent GET handler, which _does_
/// have side effects.
#[head("/authenticate")]
fn authenticate_head() -> Redirect {
    Redirect::to("/")
}

#[get("/authenticate?<token>")]
async fn authenticate(
    cookies: &CookieJar<'_>,
    auth: &State<auth::Auth>,
    token: Option<&str>,
) -> Result<Redirect, status::Unauthorized<&'static str>> {
    if let Some(t) = token {
        if let Ok(user) = auth.authenticate_token(t).await {
            // TODO: get the session token instead
            cookies.add_private(Cookie::build("session", user.user_id).secure(true).finish());
            return Ok(Redirect::to("/"));
        }
    }
    Err(status::Unauthorized(Some("Unauthorized")))
}

// TODO: Merge these structs with the trellis_web versions

#[derive(Deserialize)]
struct LoginRequest<'r> {
    email: &'r str,
}

#[derive(Serialize, Clone)]
pub struct LoginSuccess {
    email: String,
}

#[derive(Serialize, Clone)]
pub struct LoginError {
    email: String,
    message: String,
}

#[post("/login", format = "json", data = "<form>")]
async fn login(
    auth: &State<auth::Auth>,
    form: Json<LoginRequest<'_>>,
) -> Result<Json<LoginSuccess>, status::Custom<Json<LoginError>>> {
    // TODO: CSRF protection?
    if form.email.is_empty() {
        return Err(status::Custom(
            Status::BadRequest,
            Json(LoginError {
                email: form.email.to_owned(),
                message: "The email field is required.".to_owned(),
            }),
        ));
    }

    match auth.send_email(form.email).await {
        Ok(_) => Ok(Json(LoginSuccess {
            email: form.email.to_owned(),
        })),
        Err(auth::Error::Stytch(stytch::Error::Response(res)))
            if &res.error_type == "email_not_found" =>
        {
            Err(status::Custom(
                Status::BadRequest,
                Json(LoginError {
                    email: form.email.to_owned(),
                    message: format!(
                        "Sorry, {} hasn't been registered yet and signups are currently closed. Please contact a developer if you'd like to join!",
                        form.email
                    ),
                }),
            ))
        }
        Err(err) => {
            log::error!("{}", err);
            Err(status::Custom(
                Status::InternalServerError,
                Json(LoginError {
                    email: form.email.to_owned(),
                    message: "An unexpected error occurred.".to_owned(),
                }),
            ))
        }
    }
}

#[launch]
fn rocket() -> _ {
    let auth_cfg: auth::Config = Figment::from(Env::prefixed("STYTCH_"))
        .extract()
        .expect("stytch config");

    rocket::build()
        .manage(auth::Auth::new(auth_cfg).unwrap())
        .mount(
            "/v1",
            routes![load, save, authenticate, authenticate_head, login],
        )
        .attach(DbConn::fairing())
}
