use rocket;

pub mod pages;
pub mod response;
pub mod session;
pub mod users;
pub mod errors;
pub mod files;

pub fn pages() -> Vec<rocket::Route> {
    routes![
        pages::landing,
        pages::landing_public,
        pages::login,
        pages::register,
        pages::login_redirect,
        pages::register_redirect
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
