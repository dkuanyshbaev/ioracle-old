#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket_contrib;

mod config;
mod errors;
mod iching;
mod oracle;
mod wires;

use crate::errors::IOracleResult;
use config::Config;
use oracle::{ask, get};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::databases::rusqlite;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[database("ioracle")]
pub struct Db(rusqlite::Connection);

#[derive(FromForm)]
struct Question {
    email: String,
    question: String,
}

#[derive(Serialize, Debug)]
struct Answer {
    answer: String,
}

#[derive(Serialize)]
struct NoContext {}

#[get("/")]
fn index() -> Template {
    Template::render("index", NoContext {})
}

#[post("/question", data = "<question>")]
fn question(
    config: State<Config>,
    connection: Db,
    question: Option<Form<Question>>,
) -> IOracleResult<Redirect> {
    match question {
        Some(q) => Ok(Redirect::to(format!(
            "/answer/{}",
            ask(
                config,
                &connection,
                q.email.to_owned(),
                q.question.to_owned()
            )?
        ))),
        None => Ok(Redirect::to("/")),
    }
}

#[get("/answer/<uuid>")]
fn answer(connection: Db, uuid: String) -> IOracleResult<Template> {
    Ok(Template::render(
        "answer",
        Answer {
            answer: get(&connection, uuid)?,
        },
    ))
}

#[get("/operator")]
fn operator() -> Redirect {
    Redirect::to("/settings")
}

#[get("/hexagrams")]
fn hexagrams() -> Template {
    Template::render("hexagrams", NoContext {})
}

#[get("/trigrams")]
fn trigrams() -> Template {
    Template::render("trigrams", NoContext {})
}

#[get("/settings")]
fn settings() -> Template {
    Template::render("settings", NoContext {})
}

#[get("/run")]
fn run() -> Template {
    Template::render("run", NoContext {})
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", NoContext {})
}

#[catch(500)]
pub fn internal_error() -> Template {
    Template::render("500", NoContext {})
}

#[launch]
fn rocket() -> rocket::Rocket {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        std::process::exit(1);
    });

    rocket::ignite()
        .manage(config)
        .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![index, question, answer, operator, hexagrams, trigrams, settings, run],
        )
        .register(catchers![not_found, internal_error])
}
