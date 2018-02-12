use rocket;

pub mod pages;
pub mod session;
pub mod users;
pub mod errors;
pub mod files;

pub fn pages() -> Vec<rocket::Route> {
    routes![
        pages::index,
        pages::login,
        pages::index_redirect,
        pages::login_redirect
    ]
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
