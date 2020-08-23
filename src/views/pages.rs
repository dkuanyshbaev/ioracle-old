use crate::config::Config;
use crate::errors::IOracleResult;
use crate::models::binding::{Binding, UpdatedBinding};
use crate::oracle::utils::{ask_question, get_answer};
use crate::views::context::{ItemContext, NoContext};
use crate::Db;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct Question {
    email: String,
    question: String,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[post("/question", data = "<question>")]
pub fn question(
    config: State<Config>,
    connection: Db,
    question: Form<Question>,
) -> IOracleResult<Redirect> {
    Ok(Redirect::to(format!(
        "/answer/{}",
        ask_question(
            config,
            &connection,
            question.email.to_owned(),
            question.question.to_owned()
        )?
    )))
}

#[get("/answer/<uuid>")]
pub fn answer(connection: Db, uuid: String) -> IOracleResult<Template> {
    Ok(Template::render(
        "answer",
        ItemContext {
            item: get_answer(&connection, uuid)?,
        },
    ))
}

#[get("/operator")]
pub fn operator(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "operator",
        ItemContext {
            item: Binding::get(&connection)?,
        },
    ))
}

#[get("/run")]
pub fn run() -> Template {
    Template::render("run", NoContext {})
}

#[post("/save", format = "json", data = "<bindings>")]
pub fn save(connection: Db, bindings: Json<UpdatedBinding>) -> IOracleResult<Redirect> {
    Binding::update(&connection, bindings.into_inner())?;
    Binding::write_to_file(&connection)?;

    Ok(Redirect::to("/operator"))
}

#[post("/load", format = "json", data = "<bindings>")]
pub fn load(connection: Db, bindings: Json<UpdatedBinding>) -> IOracleResult<Redirect> {
    // let bindings = Binding::read_from_file("testfile.conf".to_string())?;
    Binding::update(&connection, bindings.into_inner())?;

    Ok(Redirect::to("/operator"))
}
