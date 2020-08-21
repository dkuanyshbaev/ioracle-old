use crate::errors::IOracleResult;
use crate::models::trigram::{Trigram, UpdatedTrigram};
use crate::views::context::{ItemContext, ListContext};
use crate::Db;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn all(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "trigram_list",
        ListContext {
            items: Trigram::all(&connection)?,
        },
    ))
}

#[get("/<id>")]
pub fn edit(connection: Db, id: i32) -> IOracleResult<Template> {
    Ok(Template::render(
        "trigram_form",
        ItemContext {
            item: Trigram::get(&connection, id)?,
        },
    ))
}

#[post("/<id>", data = "<trigram>")]
pub fn update(connection: Db, id: i32, trigram: Form<UpdatedTrigram>) -> IOracleResult<Redirect> {
    Trigram::update(&connection, id, trigram.into_inner())?;
    Ok(Redirect::to("/trigrams"))
}
