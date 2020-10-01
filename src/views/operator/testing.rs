use crate::oracle::wires::{off, on};
use crate::views::context::NoContext;
use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pin: u8,
    action: u8,
}

#[get("/")]
pub fn testing() -> Template {
    Template::render("testing", NoContext {})
}

#[post("/pin", format = "json", data = "<test>")]
pub fn pin(test: Json<Test>) -> Json<String> {
    match test.action {
        1 => on(test.pin),
        _ => off(test.pin),
    }
    Json("ok".to_string())
}
