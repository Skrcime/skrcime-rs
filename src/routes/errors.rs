use tera::Context;

use rocket::Request;
use rocket_contrib::Template;

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut context = Context::new();
    context.insert("path", req.uri().as_str());
    Template::render("errors/404", &context)
}

#[catch(500)]
pub fn server_error() -> Template {
    Template::render("errors/500", &Context::new())
}
