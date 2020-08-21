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
mod models;
mod oracle;
mod views;
mod wires;

use config::Config;
use rocket_contrib::databases::rusqlite::Connection;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::process::exit;
use views::operator::{hexagrams, trigrams};
use views::{catchers, pages, settings};

const DB_LOCATION: &str = "./db/ioracle.db";

#[database("ioracle")]
pub struct Db(Connection);

#[launch]
fn rocket() -> rocket::Rocket {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Can't parsing config: {}", err);
        exit(1);
    });

    let connection = Connection::open(DB_LOCATION).expect("open db");
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
                pages::run,
            ],
        )
        .mount(
            "/trigrams",
            routes![trigrams::all, trigrams::edit, trigrams::update,],
        )
        .mount(
            "/hexagrams",
            routes![hexagrams::all, hexagrams::edit, hexagrams::update,],
        )
        .mount("/settings", routes![settings::save, settings::load])
        .register(catchers![catchers::not_found, catchers::internal_error])
}
