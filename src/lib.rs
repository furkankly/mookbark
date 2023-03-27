mod common;
mod entities;
mod services;

// cli
pub mod cli;

// a local server for auth in CLI
pub fn rocket_local_mookbark_build() -> Rocket<Build> {
    use figment::{
        providers::{Format, Toml},
        Figment,
    };

    let figment = Figment::new()
        .merge(rocket::Config::default())
        .merge(Toml::file("src/cli/auth/Rocket.toml").nested())
        .merge(("tls.certs", "./ssl/cert.pem"))
        .merge(("tls.key", "./ssl/key.pem"));

    rocket::custom(figment)
        .attach(common::cors::CORS)
        .mount("/", routes![cli::auth::token::token])
        .mount("/", routes![cli::auth::page::build_dir])
        .register("/", catchers![cli::auth::page::index])
}

// server
mod server;

#[macro_use]
extern crate rocket;

use rocket::serde::json;
use rocket::{Build, Rocket};
use rocket_db_pools::Database;
#[catch(404)]
fn not_found() -> json::Value {
    json::json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn rocket_mookbark_build() -> Rocket<Build> {
    use figment::{
        providers::{
            // Env,
            Format,
            Toml,
        },
        Figment,
    };

    let figment = Figment::new()
        .merge(rocket::Config::default())
        .merge(Toml::file("src/server/Rocket.toml").nested())
        .merge(("tls.certs", "./ssl/cert.pem"))
        .merge(("tls.key", "./ssl/key.pem"));
    // .merge(Env::prefixed("ROCKET_APP_").split("_"));

    rocket::custom(figment)
        .attach(common::cors::CORS)
        .attach(server::db::Db::init())
        .mount("/", routes![server::get_bookmarks::get_bookmarks])
        .mount("/", routes![server::add_bookmark::add_bookmark])
        .mount("/", routes![server::add_container::add_container])
        .mount("/", routes![server::delete_entity::delete_entity])
        .mount("/", routes![server::token::token])
        .register("/", catchers![not_found])
}
