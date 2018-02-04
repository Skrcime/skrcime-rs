use rocket;

mod index;
mod session;
mod users;
mod errors;
mod files;

pub fn index() -> Vec<rocket::Route> {
    routes![index::index]
}
pub fn session() -> Vec<rocket::Route> {
    routes![session::create]
}
pub fn users() -> Vec<rocket::Route> {
    routes![users::create, users::get_by_email]
}
pub fn errors() -> Vec<rocket::Catcher> {
    errors![errors::not_found]
}
pub fn files() -> Vec<rocket::Route> {
    routes![files::file]
}
