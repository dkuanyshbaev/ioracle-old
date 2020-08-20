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

use config::Config;
use rocket_contrib::databases::rusqlite::Connection;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::process::exit;

#[database("ioracle")]
pub struct Db(Connection);

#[launch]
fn rocket() -> rocket::Rocket {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Can't parsing config: {}", err);
        exit(1);
    });

    let connection = Connection::open("./db/ioracle.db").expect("open db");
    db::init(&connection).unwrap_or_else(|err| {
        println!("Can't init db: {}", err);
        exit(1);
    });

    rocket::ignite()
        .manage(config)
        .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![
                pages::index,
                pages::question,
                pages::answer,
                pages::operator,
                pages::hexagrams,
                pages::trigrams,
                pages::run,
                pages::update_trigram,
                pages::edit_trigram
            ],
        )
        .register(catchers![pages::not_found, pages::internal_error])
}
