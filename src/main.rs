#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket_contrib;

mod config;
mod db;
mod errors;
mod iching;
mod oracle;
mod pages;
mod wires;

use crate::errors::IOracleResult;
use config::Config;
use oracle::{ask, get, get_hexagrams, get_trigram, get_trigrams, update_tri, TrigramEdit};
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

#[derive(Serialize)]
struct ItemContext<T> {
    item: T,
}

// #[get("/")]
// fn index() -> Template {
//     Template::render("index", NoContext {})
// }

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

#[get("/edit/trigram/<id>")]
fn edit_trigram(connection: Db, id: i32) -> IOracleResult<Template> {
    Ok(Template::render(
        "edit_trigram",
        ItemContext {
            item: get_trigram(&connection, id)?,
        },
    ))
}

#[post("/edit/trigram/<id>", data = "<trigram>")]
fn update_trigram(connection: Db, id: i32, trigram: Form<TrigramEdit>) -> IOracleResult<Redirect> {
    let t = trigram.into_inner();
    update_tri(&connection, id, t)?;

    Ok(Redirect::to("/trigrams"))
    // match trigram {
    //     Some(t) => Ok(update_tri(&connection, id, t)?),
    //     None => Ok(Redirect::to("/trigrams")),
    // }
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

    let conn = rusqlite::Connection::open("./db/ioracle.db").expect("db!");
    db::init(&conn);

    rocket::ignite()
        .manage(config)
        .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![
                pages::index,
                question,
                answer,
                operator,
                hexagrams,
                trigrams,
                run,
                update_trigram,
                edit_trigram
            ],
        )
        .register(catchers![not_found, internal_error])
}
