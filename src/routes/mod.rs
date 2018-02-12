use rocket;

mod pages;
mod session;
mod users;
mod errors;
mod files;

pub fn pages() -> Vec<rocket::Route> {
    routes![pages::index]
}
pub fn session() -> Vec<rocket::Route> {
    routes![session::create]
}
pub fn users() -> Vec<rocket::Route> {
    routes![users::create, users::get, users::update]
}
pub fn errors() -> Vec<rocket::Catcher> {
    errors![errors::not_found]
}
pub fn files() -> Vec<rocket::Route> {
    routes![files::file]
}
