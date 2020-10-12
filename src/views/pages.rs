use crate::config::Config;
use crate::errors::IOracleResult;
use crate::models::binding::{Binding, UpdatedBinding};
use crate::oracle::utils::{ask_question, get_answer};
use crate::oracle::wires::set_pwm;
use crate::views::context::{ItemContext, NoContext};
use crate::Db;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use rppal::pwm::Pwm;

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

#[post("/pwm", format = "json", data = "<pwm_data>")]
pub fn pwm(pwm_data: Json<PwmData>) -> Json<String> {
    set_pwm(pwm_data.led_freq, pwm_data.led_cycles.to_owned());
    set_pwm(pwm_data.fan_freq, pwm_data.fan_cycles.to_owned());

    Json("ok".to_string())
}
