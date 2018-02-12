use rocket_contrib::Template;
use rocket::response::Redirect;

use routes::session::Session;

static DEFAULT_TITLE: &'static str = "Skrci.me";

#[derive(Serialize)]
struct UserContext {
    title: String,
    user_id: i32,
}

#[derive(Serialize)]
struct Context {
    title: String,
}

#[get("/")]
pub fn index(session: Session) -> Template {
    Template::render(
        "index",
        &UserContext {
            title: DEFAULT_TITLE.to_string(),
            user_id: session.0,
        },
    )
}

#[get("/login", rank = 2)]
fn login() -> Template {
    Template::render(
        "login",
        &Context {
            title: DEFAULT_TITLE.to_string(),
        },
    )
}

#[get("/", rank = 2)]
fn index_redirect() -> Redirect {
    Redirect::to("/login")
}

#[get("/login")]
fn login_redirect(_session: Session) -> Redirect {
    Redirect::to("/")
}
