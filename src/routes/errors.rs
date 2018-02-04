use std::collections::HashMap;
use rocket::Request;
use rocket_contrib::Template;

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}
