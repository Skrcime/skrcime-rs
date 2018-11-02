#![cfg_attr(feature = "clippy", feature(plugin))]
#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate bcrypt;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate tera;
extern crate validator;
#[macro_use]
extern crate validator_derive;
extern crate rand;

mod db;
mod routes;
mod utils;

use db::pool;
use rocket_contrib::templates::Template;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(pool::init())
        .mount("/", routes::pages())
        .mount("/api/session", routes::session())
        .mount("/api/users", routes::users())
        .mount("/api/urls", routes::urls())
        .attach(Template::fairing())
        .register(routes::errors())
}
