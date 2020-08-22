use crate::errors::IOracleResult;
use crate::models::settings::{Settings, UpdatedSettings};
use crate::Db;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

#[post("/save", format = "json", data = "<settings>")]
pub fn save(connection: Db, settings: Json<UpdatedSettings>) -> IOracleResult<Redirect> {
    // Settings::update(&connection, settings.into_inner())?;
    // Settings::write(settings)?;

    Ok(Redirect::to("/operator"))
}

#[get("/load")]
pub fn load() -> IOracleResult<Redirect> {
    // read from file
    // save to db
    Ok(Redirect::to("/operator"))
}
