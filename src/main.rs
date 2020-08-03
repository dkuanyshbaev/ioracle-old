#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
pub struct NoContext {}

#[get("/")]
fn index() -> Template {
    Template::render("index", NoContext {})
}

#[post("/question")]
fn question() -> Redirect {
    let id = 42;
    Redirect::to(format!("/answer/{}", id))
}

#[get("/answer/<id>")]
fn answer(id: i32) -> Template {
    println!("{}", id);
    Template::render("answer", NoContext {})
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
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![index, question, answer])
        .register(catchers![not_found, internal_error])
        .launch();
}
