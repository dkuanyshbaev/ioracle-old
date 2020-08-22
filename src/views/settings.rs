use crate::errors::IOracleResult;
use crate::models::settings::Settings;
use crate::Db;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

#[post("/save", format = "json", data = "<settings>")]
pub fn save(connection: Db, settings: Json<Settings>) -> IOracleResult<Redirect> {
    settings.apply(&connection)?;
    settings.write()?;
    Ok(Redirect::to("/operator"))
}

#[get("/load")]
pub fn load() -> IOracleResult<Redirect> {
    // read from file
    // save to db
    Ok(Redirect::to("/operator"))
}
