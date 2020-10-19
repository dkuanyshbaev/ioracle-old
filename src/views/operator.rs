use crate::errors::IOracleResult;
use crate::models::binding::Binding;
use crate::wires::{
    build_controller, colour_off, colour_on, fire_on, pin_off, pin_on, play_sound, reset_all,
    run_simulation,
};
use crate::Db;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pin: u8,
    colour: String,
    code: String,
    action: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sound {
    file_name: String,
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
    if test.code == "fire" {
        fire_on();
    } else {
        match test.action {
            1 => colour_on(test.colour.to_owned(), test.code.to_owned()),
            _ => colour_off(),
        }
    };

    Json("ok".to_string())
}

#[post("/sound", format = "json", data = "<sound>")]
pub fn sound(sound: Json<Sound>) -> Json<String> {
    play_sound(sound.file_name.to_owned());

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
