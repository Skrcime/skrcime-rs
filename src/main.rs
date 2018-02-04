#![cfg_attr(feature = "clippy", feature(plugin))]
#![feature(plugin)]
#![plugin(rocket_codegen)]

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
extern crate uuid;

mod db;
mod routes;

use db::pool;
use rocket_contrib::Template;

fn main() {
    rocket::ignite()
        .manage(pool::init())
        .mount("/", routes::index())
        .mount("/session", routes::session())
        .mount("/users", routes::users())
        .mount("/static", routes::files())
        .attach(Template::fairing())
        .catch(routes::errors())
        .launch();
}
