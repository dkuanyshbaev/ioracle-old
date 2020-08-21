use crate::errors::IOracleResult;
use rocket::response::Redirect;

#[get("/save")]
pub fn save() -> IOracleResult<Redirect> {
    Ok(Redirect::to("/operator"))
}

#[get("/load")]
pub fn load() -> IOracleResult<Redirect> {
    Ok(Redirect::to("/operator"))
}
