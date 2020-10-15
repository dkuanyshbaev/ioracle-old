use crate::config::Config;
use crate::errors::IOracleResult;
use crate::models::binding::{Binding, UpdatedBinding};
use crate::models::hexagram::UpdatedHexagram;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PwmData {
    led_freq: i32,
    led_cycles: String,
    fan_freq: i32,
    fan_cycles: String,
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
    let mut hcount = 0;
    for result in reader.deserialize() {
        let record: UpdatedHexagram = result?;
        println!("{:#?}", record);
        hcount += 1;
    }

    //trigrams
    // let file_path = "/home/denis/collector/iora/csv/expanded_gua.csv";
    // let mut reader = csv::Reader::from_path(file_path)?;
    // let mut tcount = 0;
    // for result in reader.deserialize() {
    //     let record: UpdatedTrigram = result?;
    //     println!("{:#?}", record);
    //     tcount += 1;
    // }

    println!("hexagrams count: {}", hcount);
    // println!("trigrams count: {}", tcount);
    Ok("Ok".to_string())
}
