use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct NoContext {}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}
