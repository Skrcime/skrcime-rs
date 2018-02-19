use rocket_contrib::Template;
use rocket::response::Redirect;

use tera::Context;

use routes::session::Session;

#[get("/")]
pub fn landing(_session: Session) -> Template {
    Template::render("landing", Context::new())
}

#[get("/prijava", rank = 2)]
fn login() -> Template {
    Template::render("login", Context::new())
}

#[get("/registracija", rank = 2)]
fn register() -> Template {
    Template::render("register", Context::new())
}

#[get("/", rank = 2)]
fn landing_redirect() -> Redirect {
    Redirect::to("/prijava")
}

#[get("/prijava")]
fn login_redirect(_session: Session) -> Redirect {
    Redirect::to("/")
}
#[get("/registracija")]
fn register_redirect(_session: Session) -> Redirect {
    Redirect::to("/")
}
