#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::http::{Cookie, CookieJar};
use rocket::response::{status, Redirect};
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use serde_json::Value; // TODO: Use an auto-migrating version struct

mod auth;
pub mod models;
pub mod schema;

#[database("trellis")]
struct DbConn(PgConnection);

#[get("/load")]
async fn load(
    db: DbConn,
    cookies: &CookieJar<'_>,
) -> Result<Option<Json<Value>>, status::Unauthorized<&'static str>> {
    use schema::settings::dsl::*;

    let uid = match cookies.get_private("session") {
        None => return Err(status::Unauthorized(Some("Unauthorized"))),
        Some(cookie) => String::from(cookie.value()),
    };

    let res: QueryResult<models::Settings> = db
        .run(move |c| {
            settings
                .filter(user_id.eq(uid))
                .first::<models::Settings>(c)
        })
        .await;

    match res {
        Ok(row) => Ok(Some(Json(row.data))),
        Err(err) => {
            // TODO: Real logging
            println!("{}", err);
            Ok(None)
        }
    }
}

#[post("/save", format = "json", data = "<data>")]
async fn save(
    db: DbConn,
    cookies: &CookieJar<'_>,
    data: Json<Value>,
) -> Result<&'static str, status::Unauthorized<&'static str>> {
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
            // TODO: This needs to be an upsert lol
            diesel::insert_into(schema::settings::table)
                .values(&new_settings)
                .on_conflict(dsl::user_id)
                .do_update()
                .set(dsl::data.eq(&new_settings.data))
                .execute(c)
        })
        .await;

    if let Err(err) = res {
        // TODO: Real logging
        println!("{}", err);
        return Err(status::Unauthorized(Some("Unauthorized")));
    }

    // TODO: Meaningful return value
    Ok("{}")
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
            // TODO: Set `secure` when serving HTTPS
            cookies.add_private(Cookie::new("session", user.user_id));
            return Ok(Redirect::to("/"));
        }
    }
    Err(status::Unauthorized(Some("Unauthorized")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![load, save])
        .mount("/", routes![authenticate, authenticate_head])
        .attach(DbConn::fairing())
}
