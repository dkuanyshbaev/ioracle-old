use crate::errors::IOracleResult;
use crate::models::hexagram::{Hexagram, UpdatedHexagram};
use crate::views::context::{ItemContext, ListContext};
use crate::Db;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn all(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "hexagram_list",
        ListContext {
            items: Hexagram::all(&connection)?,
        },
    ))
}

#[get("/<id>")]
pub fn edit(connection: Db, id: i32) -> IOracleResult<Template> {
    Ok(Template::render(
        "hexagram_form",
        ItemContext {
            item: Hexagram::get(&connection, id)?,
        },
    ))
}

#[post("/<id>", data = "<hexagram>")]
pub fn update(connection: Db, id: i32, hexagram: Form<UpdatedHexagram>) -> IOracleResult<Redirect> {
    Hexagram::update(&connection, id, hexagram.into_inner())?;
    Ok(Redirect::to("/hexagrams"))
}
