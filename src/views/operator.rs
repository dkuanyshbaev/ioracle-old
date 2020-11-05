use crate::errors::IOracleResult;
use crate::models::binding::Binding;
use crate::wires::{
    build_controller, colour_off, colour_on, fire_on, li_off, li_on, open_pip, pin7_start,
    pin8_start, pin_off, pin_on, play_sound, reset_all, resting_off, resting_on, run_emulation,
    run_simulation, shell_fire, shimmering_on,
};
use crate::Db;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pin: u8,
    colour: String,
    code: String,
    action: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rest {
    yao_colour: String,
    li_colour: String,
    action: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Li {
    li_colour: String,
    resting_colour: String,
    action: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sound {
    file_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pip {
    multiply: String,
    bias: String,
    threshold: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trigrams {
    first_trigram: String,
    second_trigram: String,
    li_colour: String,
}

#[post("/pin", format = "json", data = "<test>")]
pub fn pin(test: Json<Test>) -> Json<String> {
    if test.pin == 7 {
        pin7_start();
    } else if test.pin == 8 {
        pin8_start();
    } else {
        match test.action {
            1 => pin_on(test.pin),
            _ => pin_off(test.pin),
        }
    }

    Json("ok".to_string())
}

#[post("/colour", format = "json", data = "<test>")]
pub fn colour(test: Json<Test>) -> Json<String> {
    if test.code == "fire" {
        // fire_on();
        shell_fire();
    } else if test.code == "shim" {
        shimmering_on();
    } else {
        match test.action {
            1 => colour_on(test.colour.to_owned(), test.code.to_owned()),
            _ => colour_off(),
        }
    };

    Json("ok".to_string())
}

#[post("/resting", format = "json", data = "<rest>")]
pub fn resting(rest: Json<Rest>) -> Json<String> {
    match rest.action {
        1 => resting_on(rest.yao_colour.to_owned(), rest.li_colour.to_owned()),
        _ => resting_off(),
    }

    Json("ok".to_string())
}

#[post("/li", format = "json", data = "<li>")]
pub fn li(li: Json<Li>) -> Json<String> {
    match li.action {
        1 => li_on(li.li_colour.to_owned(), li.resting_colour.to_owned()),
        _ => li_off(),
    }

    Json("ok".to_string())
}

#[post("/sound", format = "json", data = "<sound>")]
pub fn sound(sound: Json<Sound>) -> Json<String> {
    play_sound(sound.file_name.to_owned());

    Json("ok".to_string())
}

#[post("/pip", format = "json", data = "<pip>")]
pub fn pip(pip: Json<Pip>) -> Json<String> {
    open_pip(
        pip.multiply.to_owned(),
        pip.bias.to_owned(),
        pip.threshold.to_owned(),
    );

    Json("ok".to_string())
}

#[get("/simulation")]
pub fn simulation(connection: Db) -> IOracleResult<Json<String>> {
    run_simulation(Binding::get(&connection)?)?;

    Ok(Json("ok".to_string()))
}

#[post("/emulation", format = "json", data = "<trigrams>")]
pub fn emulation(connection: Db, trigrams: Json<Trigrams>) -> IOracleResult<Json<String>> {
    run_emulation(
        &Binding::get(&connection)?,
        &trigrams.first_trigram,
        &trigrams.second_trigram,
    )?;

    Ok(Json("ok".to_string()))
}

#[get("/reset")]
pub fn reset(connection: Db) -> IOracleResult<Json<String>> {
    let settings = Binding::get(&connection)?;
    let mut controller = build_controller()?;
    reset_all(&settings, &mut controller);

    Ok(Json("ok".to_string()))
}

#[get("/send")]
pub fn send() -> IOracleResult<Json<String>> {
    // pub fn send() -> Redirect {
    println!("send to gate");

    use std::io::prelude::*;
    use std::io::{BufRead, BufReader};
    use std::os::unix::net::{UnixListener, UnixStream};
    // let mut stream = UnixStream::connect("/tmp/ioracle.gate")?;
    if let Ok(mut stream) = UnixStream::connect("/tmp/ioracle.gate") {
        match stream.write_all(b"read") {
            Ok(_) => {
                // let listener = UnixListener::bind("/tmp/ioracle.out").unwrap();
                // for stream in listener.incoming() {
                //     match stream {
                //         Ok(stream) => {
                //             let stream = BufReader::new(stream);
                //             for line in stream.lines() {
                //                 println!("{}", line.unwrap());
                //             }
                //         }
                //         Err(err) => {
                //             println!("Error: {}", err);
                //             break;
                //         }
                //     }
                // }
                // if let Ok(mut out) = UnixStream::connect("/tmp/ioracle.out") {
                //     let mut response = String::new();
                //     match out.read_to_string(&mut response) {
                //         Ok(b) => {
                //             println!("{:?}", b);
                //             println!("{}", response);
                //         }
                //         Err(e) => println!("{:?}", e),
                //     };
                // }
                // let mut response = String::new();
                // match stream.read_to_string(&mut response) {
                //     Ok(b) => {
                //         println!("{:?}", b);
                //         println!("{}", response);
                //     }
                //     Err(e) => println!("{:?}", e),
                // };
            }
            Err(e) => println!("{:?}", e),
        };
    };

    // Redirect::to("/operator/result")
    // Ok(Json("ok".to_string()))

    println!("show result");

    // use std::io::prelude::*;
    // use std::io::{BufRead, BufReader};
    // use std::os::unix::net::{UnixListener, UnixStream};

    use std::fs;
    use std::path::Path;
    let socket = Path::new("/tmp/ioracle.out");
    // Delete old socket if necessary
    if socket.exists() {
        // fs::unlink(&socket).unwrap();
        if let Err(error) = std::fs::remove_file("/tmp/ioracle.out") {
            println!("{}", error);
            std::process::exit(1);
        };
    }
    // let listener = UnixListener::bind("/tmp/ioracle.out").unwrap_or_else(|error| {
    //     println!("{}", error);
    //     std::process::exit(1);
    // });

    let listener = UnixListener::bind("/tmp/ioracle.out").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = BufReader::new(stream);

                println!("{:?}", stream.lines().nth(0));

                // for line in stream.lines() {
                //     println!("{}", line.unwrap());
                //     break;
                // }
                break;
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }

    Ok(Json("ok".to_string()))
}

#[get("/result")]
pub fn result() -> IOracleResult<Json<String>> {
    println!("show result");

    use std::io::prelude::*;
    use std::io::{BufRead, BufReader};
    use std::os::unix::net::{UnixListener, UnixStream};

    use std::fs;
    use std::path::Path;
    let socket = Path::new("/tmp/ioracle.out");
    // Delete old socket if necessary
    if socket.exists() {
        // fs::unlink(&socket).unwrap();
        if let Err(error) = std::fs::remove_file("/tmp/ioracle.out") {
            println!("{}", error);
            std::process::exit(1);
        };
    }
    // let listener = UnixListener::bind("/tmp/ioracle.out").unwrap_or_else(|error| {
    //     println!("{}", error);
    //     std::process::exit(1);
    // });

    let listener = UnixListener::bind("/tmp/ioracle.out").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = BufReader::new(stream);

                println!("{:?}", stream.lines().nth(0));

                // for line in stream.lines() {
                //     println!("{}", line.unwrap());
                //     break;
                // }
                break;
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }

    Ok(Json("ok".to_string()))
}
