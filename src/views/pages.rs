use crate::config::Config;
use crate::errors::IOracleResult;
use crate::models::binding::{Binding, UpdatedBinding};
use crate::models::hexagram::{Hexagram, UpdatedHexagram};
use crate::models::record::Record;
use crate::models::trigram::UpdatedTrigram;
use crate::oracle::ask;
use crate::views::context::{AnswerContext, ItemContext, NoContext};
use crate::wires::reset;
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
pub fn index(connection: Db) -> Template {
    if let Ok(settings) = Binding::get(&connection) {
        reset(settings);
    };
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
        ask(
            config,
            &connection,
            question.email.to_owned(),
            question.question.to_owned()
        )?
    )))
}

#[get("/answer/<uuid>")]
pub fn answer(connection: Db, uuid: String) -> IOracleResult<Template> {
    let record = Record::get_by_uuid(&connection, uuid)?;
    let hexagram = Hexagram::get_by_binary(&connection, record.hexagram.clone())?;
    let related = Hexagram::get_by_binary(&connection, record.related.clone())?;

    Ok(Template::render(
        "answer",
        AnswerContext {
            record,
            hexagram,
            related,
        },
    ))
}

#[get("/settings")]
pub fn settings(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "settings",
        ItemContext {
            item: Binding::get(&connection)?,
        },
    ))
}

#[post("/save", format = "json", data = "<bindings>")]
pub fn save(connection: Db, bindings: Json<UpdatedBinding>) -> IOracleResult<Redirect> {
    Binding::update(&connection, bindings.into_inner())?;

    Ok(Redirect::to("/settings"))
}

#[get("/csv")]
pub fn csv(_connection: Db) -> IOracleResult<String> {
    // hexagrams
    let file_path = "/home/denis/collector/iora/csv/expanded_gua.csv";
    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.deserialize() {
        let record: UpdatedHexagram = result?;
        println!("{:#?}", record);
        // let _h = crate::models::hexagram::Hexagram::insert(&connection, record)?;
    }

    //trigrams
    let file_path = "/home/denis/collector/iora/csv/trigrams.csv";
    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.deserialize() {
        let record: UpdatedTrigram = result?;
        println!("{:#?}", record);
        // let _t = crate::models::trigram::Trigram::insert(&connection, record)?;
    }

    Ok("Ok".to_string())
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/")
}

#[catch(500)]
pub fn internal_error() -> Redirect {
    Redirect::to("/")
}
