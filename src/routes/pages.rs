use rocket_contrib::Template;
use rocket::response::Redirect;
use diesel::prelude::*;

use tera::Context;

use db::request::DbConnection;
use db::models::{User, Url};

use routes::session::Session;
use routes::response::user_to_json;
use routes::errors::server_error;

#[get("/")]
pub fn landing(session: Session, conn: DbConnection) -> Result<Template, Template> {
    use db::schema::users::dsl;

    dsl::users
        .filter(dsl::id.eq(&session.0))
        .first::<User>(&*conn)
        .map(|user: User| {
            let mut context = Context::new();
            context.add("user", &user);
            context.add("user_json", &user_to_json(user).to_string());
            Template::render("pages/landing", context)
        })
        .map_err(|_err| server_error())
}
#[get("/", rank = 2)]
fn landing_public() -> Template {
    Template::render("pages/landing", Context::new())
}

#[get("/prijava", rank = 2)]
fn login() -> Template {
    Template::render("pages/login", Context::new())
}

#[get("/registracija", rank = 2)]
fn register() -> Template {
    Template::render("pages/register", Context::new())
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

    let url = dsl::urls.filter(dsl::hash.eq(&hash)).first::<Url>(&*conn);
    match url {
        Ok(url) => Redirect::to(&url.target),
        Err(_) => Redirect::to("/"),
    }
}
