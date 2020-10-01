use crate::errors::IOracleResult;
use crate::models::binding::Binding;
use crate::oracle::wires::{pin_off, pin_on, run_simulation};
use crate::views::context::NoContext;
use crate::Db;
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
        1 => pin_on(test.pin),
        _ => pin_off(test.pin),
    }
    Json("ok".to_string())
}

#[get("/simulation")]
pub fn simulation(connection: Db) -> IOracleResult<Json<String>> {
    run_simulation(Binding::get(&connection)?);

    Ok(Json("ok".to_string()))
}
