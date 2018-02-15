use rocket_contrib::Template;
use rocket::response::Redirect;

use routes::session::Session;

static DEFAULT_TITLE: &'static str = "Skrci.me";

#[derive(Serialize)]
struct Context {
    title: String,
}

#[get("/")]
pub fn index(_session: Session) -> Template {
    Template::render(
        "index",
        &Context {
            title: DEFAULT_TITLE.to_string(),
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
