#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

// #[macro_use]
// extern crate diesel_migrations;

mod config;
mod errors;
mod models;
mod oracle;
mod views;

use config::Config;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rppal::pwm::{Channel, Polarity, Pwm};
use views::operator::{hexagrams, testing, trigrams};
use views::{catchers, pages};

// embed_migrations!();

#[database("ioracle")]
pub struct Db(diesel::SqliteConnection);

// async fn run_migrations(mut rocket: Rocket) -> Result<Rocket, Rocket> {
//     let conn = Db::get_one(rocket.inspect().await).expect("database connection");
//     match embedded_migrations::run(&*conn) {
//         Ok(()) => Ok(rocket),
//         Err(e) => {
//             println!("Failed to run database migrations: {:?}", e);
//             Err(rocket)
//         }
//     }
// }

// #[launch]
// fn rocket() -> Rocket {
//     let config = Config::new().unwrap_or_else(|err| {
//         println!("Can't parsing config: {}", err);
//         std::process::exit(1);
//     });
//
//     // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) at 2 Hz with a 25% duty cycle.
//     let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true).unwrap_or_else(
//         |err| {
//             println!("Can't init pwm channels: {}", err);
//             std::process::exit(1);
//         },
//     );
//
//     rocket::ignite()
//         .manage(config)
//         .manage(pwm)
//         .attach(Db::fairing())
//         .attach(AdHoc::on_attach("run migrations", run_migrations))
//         .attach(Template::fairing())
//         .mount("/static", StaticFiles::from("static/"))
//         .mount(
//             "/",
//             routes![
//                 pages::index,
//                 pages::question,
//                 pages::answer,
//                 pages::settings,
//                 pages::save,
//                 pages::pwm
//             ],
//         )
//         .mount(
//             "/trigrams",
//             routes![trigrams::all, trigrams::edit, trigrams::update],
//         )
//         .mount(
//             "/hexagrams",
//             routes![hexagrams::all, hexagrams::edit, hexagrams::update],
//         )
//         .mount(
//             "/testing",
//             routes![testing::element, testing::simulation, testing::reset],
//         )
//         .register(catchers![catchers::not_found, catchers::internal_error])
// }

fn rocket(config: Config) -> rocket::Rocket {
    rocket::ignite()
        .manage(config)
        // .manage(pwm)
        .attach(Db::fairing())
        // .attach(AdHoc::on_attach("run migrations", run_migrations))
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![
                pages::index,
                pages::question,
                pages::answer,
                pages::settings,
                pages::save,
                pages::pwm
            ],
        )
        .mount(
            "/trigrams",
            routes![trigrams::all, trigrams::edit, trigrams::update],
        )
        .mount(
            "/hexagrams",
            routes![hexagrams::all, hexagrams::edit, hexagrams::update],
        )
        .mount(
            "/testing",
            routes![testing::element, testing::simulation, testing::reset],
        )
        .register(catchers![catchers::not_found, catchers::internal_error])
}

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        std::process::exit(1);
    });

    let error = rocket(config).launch();
    println!("Launch failed: {}", error);
}
