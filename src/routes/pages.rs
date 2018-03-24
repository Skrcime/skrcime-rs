use rocket_contrib::Template;
use rocket::response::Redirect;

use diesel::prelude::*;

use tera::Context;

use routes::session::Session;

use db::request::DbConnection;
use db::models::Url;

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

#[get("/<hash>", rank = 3)]
pub fn redirect(conn: DbConnection, hash: String) -> Redirect {
    use db::schema::urls::dsl;

    println!("hash {:?}", hash);
    let url = dsl::urls.filter(dsl::hash.eq(&hash)).first::<Url>(&*conn);
    match url {
        Ok(url) => Redirect::to(&url.target),
        Err(_) => Redirect::to("/"),
    }
}
