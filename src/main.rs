#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

mod config;
mod errors;
mod iching;
mod models;
mod oracle;
mod views;
mod wires;

use config::Config;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use views::{operator, pages};

#[database("ioracle")]
pub struct Db(diesel::SqliteConnection);

fn rocket(config: Config) -> rocket::Rocket {
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
                pages::save,
                pages::csv,
            ],
        )
        .mount(
            "/operator",
            routes![
                operator::pin,
                operator::colour,
                operator::li,
                operator::resting,
                operator::sound,
                operator::pip,
                operator::simulation,
                operator::emulation,
                operator::reset,
                operator::result,
                operator::send
            ],
        )
        .register(catchers![pages::not_found, pages::internal_error])
}

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        std::process::exit(1);
    });

    let error = rocket(config).launch();
    println!("Launch failed: {}", error);
}
