use rocket;

pub mod pages;
pub mod session;
pub mod users;
pub mod errors;
pub mod files;

pub fn pages() -> Vec<rocket::Route> {
    routes![
        pages::landing,
        pages::login,
        pages::landing_redirect,
        pages::login_redirect
    ]
}
pub fn session() -> Vec<rocket::Route> {
    routes![session::create]
}
pub fn users() -> Vec<rocket::Route> {
    routes![
        users::create,
        users::get,
        users::get_401,
        users::update,
        users::update_401
    ]
}
pub fn errors() -> Vec<rocket::Catcher> {
    errors![errors::not_found]
}
pub fn files() -> Vec<rocket::Route> {
    routes![files::file]
}
