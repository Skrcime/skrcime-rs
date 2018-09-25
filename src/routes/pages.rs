use diesel::prelude::*;
use rocket::response::Redirect;
use rocket_contrib::Template;

use tera::Context;

use db::models::User;
use db::request::DbConnection;

use routes::errors::server_error;
use routes::response::user_to_json;
use routes::session::Session;

#[get("/")]
pub fn landing(session: Session, conn: DbConnection) -> Result<Template, Template> {
    use db::schema::users::dsl;

    dsl::users
        .filter(dsl::id.eq(&session.0))
        .first::<User>(&*conn)
        .map(|user: User| {
            let mut context = Context::new();
            context.insert("user", &user);
            context.insert("user_json", &user_to_json(user).to_string());
            Template::render("pages/landing", context)
        }).map_err(|_err| server_error())
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
