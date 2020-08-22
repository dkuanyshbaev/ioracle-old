use crate::errors::IOracleResult;
use crate::models::settings::Settings;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

#[post("/save", format = "json", data = "<settings>")]
pub fn save(settings: Json<Settings>) -> IOracleResult<Redirect> {
    println!("------------------- {:?}", settings);
    Ok(Redirect::to("/operator"))
}

#[get("/load")]
pub fn load() -> IOracleResult<Redirect> {
    // read from file
    // save to db
    Ok(Redirect::to("/operator"))
}
