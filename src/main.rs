#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket_contrib;

mod lib;

use lib::{ask, get_answer};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[database("ioracle")]
pub struct Db(rusqlite::Connection);

#[derive(FromForm)]
struct Question {
    email: String,
    question: String,
}

#[derive(Serialize)]
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
fn question(connection: Db, question: Option<Form<Question>>) -> Redirect {
    match question {
        Some(q) => {
            let answer_uuid = ask(&connection, q.email.to_owned(), q.question.to_owned());
            Redirect::to(format!("/answer/{}", answer_uuid))
        }
        None => Redirect::to("/"),
    }
}

#[get("/answer/<uuid>")]
fn answer(connection: Db, uuid: String) -> Option<Template> {
    Some(Template::render(
        "answer",
        Answer {
            answer: get_answer(&connection, uuid)?,
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

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(Db::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![index, question, answer])
        .register(catchers![not_found, internal_error])
        .launch();
}
