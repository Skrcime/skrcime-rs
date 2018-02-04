use rocket_contrib::Template;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", ())
}
