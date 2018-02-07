#![cfg_attr(feature = "clippy", feature(plugin))]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(test, plugin(stainless))]

extern crate bcrypt;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod db;
mod routes;

use db::pool;
use rocket_contrib::Template;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(pool::init())
        .mount("/", routes::index())
        .mount("/session", routes::session())
        .mount("/users", routes::users())
        .mount("/static", routes::files())
        .attach(Template::fairing())
        .catch(routes::errors())
}
