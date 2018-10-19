use diesel::prelude::*;
use rocket::response::Redirect;
use rocket_contrib::Template;

use tera::Context;

use db::models::User;
use db::request::DbConnection;

use routes::errors::server_error;
use routes::response::user_to_json;
use routes::session::Session;
use routes::subdomain::Subdomain;

#[get("/")]
fn subdomain(
    session: Session,
    subdomain: Subdomain,
    conn: DbConnection,
) -> Result<Template, Template> {
    user_page(session.0, &subdomain.0, conn)
}
#[get("/", rank = 2)]
fn landing(session: Session, conn: DbConnection) -> Result<Template, Template> {
    user_page(session.0, "landing", conn)
}
#[get("/", rank = 3)]
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

fn user_page(user_id: i32, page: &str, conn: DbConnection) -> Result<Template, Template> {
    use db::schema::users::dsl;

    dsl::users
        .filter(dsl::id.eq(user_id))
        .first::<User>(&*conn)
        .map(|user: User| {
            let mut context = Context::new();
            context.insert("user", &user);
            context.insert("user_json", &user_to_json(user).to_string());
            Template::render(format!("pages/{}", page), context)
        })
        .map_err(|_err| server_error())
}
