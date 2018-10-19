use rocket;

pub mod errors;
pub mod files;
pub mod pages;
pub mod redirect;
pub mod response;
pub mod session;
pub mod subdomain;
pub mod urls;
pub mod users;

pub fn pages() -> Vec<rocket::Route> {
    routes![
        pages::subdomain,
        pages::landing,
        pages::landing_public,
        pages::login,
        pages::register,
        pages::login_redirect,
        pages::register_redirect,
        redirect::redirect
    ]
}
pub fn session() -> Vec<rocket::Route> {
    routes![session::create, session::destroy]
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
pub fn urls() -> Vec<rocket::Route> {
    routes![urls::create_user, urls::create_public, urls::get_all]
}
pub fn errors() -> Vec<rocket::Catcher> {
    catchers![errors::not_found, errors::server_error]
}
pub fn files() -> Vec<rocket::Route> {
    routes![files::file]
}
