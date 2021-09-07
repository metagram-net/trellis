#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use anyhow;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::{status, Redirect};
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
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
    token: Option<&str>,
) -> Result<Redirect, status::Unauthorized<&'static str>> {
    if let Some(t) = token {
        if let Ok(user) = auth::authenticate(t) {
            cookies.add_private(Cookie::build("session", user.user_id).secure(true).finish());
            return Ok(Redirect::to("/"));
        }
    }
    Err(status::Unauthorized(Some("Unauthorized")))
}

#[get("/health-check")]
fn health_check() -> &'static str {
    ""
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![load, save])
        .mount("/", routes![authenticate, authenticate_head])
        .mount("/.well-known", routes![health_check])
        .attach(DbConn::fairing())
}
