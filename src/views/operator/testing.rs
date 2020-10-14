use crate::errors::IOracleResult;
use crate::models::binding::Binding;
use crate::oracle::utils::send;
use crate::oracle::wires::{
    build_controller, colour_off, colour_on, element_off, element_on, pin_off, pin_on, reset_all,
    run_simulation,
};
use crate::{Config, Db};
use rocket::State;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pin: u8,
    colour: String,
    code: String,
    action: u8,
}

#[post("/element", format = "json", data = "<test>")]
pub fn element(test: Json<Test>) -> Json<String> {
    match test.action {
        1 => element_on(test.pin, test.colour.to_owned(), test.code.to_owned()),
        _ => element_off(test.pin),
    }

    Json("ok".to_string())
}

#[post("/pin", format = "json", data = "<test>")]
pub fn pin(test: Json<Test>) -> Json<String> {
    match test.action {
        1 => pin_on(test.pin),
        _ => pin_off(test.pin),
    }

    Json("ok".to_string())
}

#[post("/colour", format = "json", data = "<test>")]
pub fn colour(test: Json<Test>) -> Json<String> {
    match test.action {
        1 => colour_on(test.colour.to_owned(), test.code.to_owned()),
        _ => colour_off(),
    }

    Json("ok".to_string())
}

#[get("/simulation")]
pub fn simulation(connection: Db) -> IOracleResult<Json<String>> {
    run_simulation(Binding::get(&connection)?)?;

    Ok(Json("ok".to_string()))
}

#[get("/reset")]
pub fn reset(connection: Db) -> IOracleResult<Json<String>> {
    let settings = Binding::get(&connection)?;
    let mut controller = build_controller()?;
    reset_all(&settings, &mut controller);

    Ok(Json("ok".to_string()))
}

#[get("/mail")]
pub fn mail(config: State<Config>) -> IOracleResult<Json<String>> {
    println!("mail---------------->");
    let email = "konsistentsi@gmail.com".to_string();
    let question = "test".to_string();
    let answer = "!".to_string();

    send(config, &email, &question, &answer)?;

    Ok(Json("ok".to_string()))
}
