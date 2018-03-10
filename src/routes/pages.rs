use rocket_contrib::Template;
use rocket::response::Redirect;
use diesel::prelude::*;

use tera::Context;

use db::request::DbConnection;
use db::models::{User};
use routes::session::Session;

#[get("/")]
pub fn landing(session: Session, conn: DbConnection) -> Result<Template, Redirect> {
    use db::schema::users::dsl;

    dsl::users
        .filter(dsl::id.eq(&session.0))
        .first::<User>(&*conn)
        .map(|user: User| {
            let mut context = Context::new();
            let user_json = &json!({
                "id": user.id,
                "name": user.name,
                "email": user.email,
                "admin": user.admin,
                "welcome": user.welcome,
                "avatar_url": user.avatar_url,
                "created_at": user.created_at,
            });
            context.add("user_json", &user_json.to_string());
            Template::render("landing", context)
        })
        .map_err(|_err| {
            Redirect::to("/") // TODO: redirect to 500 page
        })
}
#[get("/", rank = 2)]
fn landing_public() -> Template {
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

#[get("/prijava")]
fn login_redirect(_session: Session) -> Redirect {
    Redirect::to("/")
}
#[get("/registracija")]
fn register_redirect(_session: Session) -> Redirect {
    Redirect::to("/")
}
