use rocket_contrib::Template;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
}

#[get("/")]
pub fn index() -> Template {
    let context = TemplateContext {
        title: "skrci.me".to_string(),
    };
    Template::render("index", &context)
}
