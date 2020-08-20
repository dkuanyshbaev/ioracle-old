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
use oracle::{ask, get, get_hexagrams, get_trigrams, init_db};
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

#[derive(Serialize)]
struct ListContext<T> {
    items: Vec<T>,
}

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
fn operator() -> Template {
    Template::render("operator", NoContext {})
}

#[get("/init")]
fn init(connection: Db) -> IOracleResult<Redirect> {
    init_db(&connection)?;

    Ok(Redirect::to("/operator"))
}

#[get("/run")]
fn run() -> Template {
    Template::render("run", NoContext {})
}

#[get("/trigrams")]
fn trigrams(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "trigrams",
        ListContext {
            items: get_trigrams(&connection)?,
        },
    ))
}

#[get("/hexagrams")]
fn hexagrams(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "hexagrams",
        ListContext {
            items: get_hexagrams(&connection)?,
        },
    ))
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
            routes![index, question, answer, operator, hexagrams, trigrams, init, run],
        )
        .register(catchers![not_found, internal_error])
}
