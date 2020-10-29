use crate::config::Config;
use crate::errors::IOracleResult;
use crate::models::binding::{Binding, UpdatedBinding};
use crate::models::hexagram::{Hexagram, SheetsHexagram, UpdatedHexagram};
use crate::models::record::Record;
use crate::models::trigram::{Trigram, UpdatedTrigram};
use crate::oracle::ask;
use crate::wires::reset;
use crate::Db;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct Question {
    email: String,
    question: String,
}

#[derive(Serialize)]
pub struct ItemContext<T> {
    pub item: T,
}

#[derive(Serialize)]
pub struct ListContext<T> {
    pub items: Vec<T>,
}

#[derive(Serialize)]
pub struct NoContext {}

#[derive(Serialize)]
pub struct AnswerContext<R, T, H> {
    pub record: R,
    pub hexagram: H,
    pub related: H,
    pub core_primary: H,
    pub core_related: H,
    pub first_trigram: T,
    pub second_trigram: T,
    pub first_related: T,
    pub second_related: T,
    pub core_primary_first: T,
    pub core_primary_second: T,
    pub core_related_first: T,
    pub core_related_second: T,
}

#[get("/")]
pub fn index(connection: Db) -> Template {
    if let Ok(settings) = Binding::get(&connection) {
        reset(settings);
    };
    Template::render("index", NoContext {})
}

// #[post("/question", data = "<question>")]
// pub fn question(
//     config: State<Config>,
//     connection: Db,
//     question: Form<Question>,
// ) -> IOracleResult<Redirect> {
//     Ok(Redirect::to(format!(
//         "/answer/{}",
//         ask(
//             config,
//             &connection,
//             question.email.to_owned(),
//             question.question.to_owned()
//         )?
//     )))
// }

#[get("/answer/<uuid>")]
pub fn answer(connection: Db, uuid: String) -> IOracleResult<Template> {
    let record = Record::get_by_uuid(&connection, uuid)?;

    let h_binary = record.hexagram.clone();
    let r_binary = record.related.clone();

    let hexagram = Hexagram::get_by_binary(&connection, h_binary.clone())?;
    let related = Hexagram::get_by_binary(&connection, r_binary.clone())?;

    let first_trigram = Trigram::get_by_binary(&connection, &(&h_binary[..3]).to_string())?;
    let second_trigram = Trigram::get_by_binary(&connection, &(&h_binary[3..]).to_string())?;
    let first_related = Trigram::get_by_binary(&connection, &(&r_binary[..3]).to_string())?;
    let second_related = Trigram::get_by_binary(&connection, &(&r_binary[3..]).to_string())?;

    let core_primary_first = Trigram::get_by_binary(&connection, &(&h_binary[1..4]).to_string())?;
    let core_primary_second = Trigram::get_by_binary(&connection, &(&h_binary[2..5]).to_string())?;
    let core_related_first = Trigram::get_by_binary(&connection, &(&r_binary[1..4]).to_string())?;
    let core_related_second = Trigram::get_by_binary(&connection, &(&r_binary[2..5]).to_string())?;

    let core_p_binary = format!("{}{}", &h_binary[1..4], &h_binary[2..5]);
    let core_r_binary = format!("{}{}", &r_binary[1..4], &r_binary[2..5]);
    let core_primary = Hexagram::get_by_binary(&connection, core_p_binary)?;
    let core_related = Hexagram::get_by_binary(&connection, core_r_binary)?;

    Ok(Template::render(
        "answer",
        AnswerContext {
            record,
            hexagram,
            related,
            first_trigram,
            second_trigram,
            first_related,
            second_related,
            core_primary,
            core_related,
            core_primary_first,
            core_primary_second,
            core_related_first,
            core_related_second,
        },
    ))
}

#[get("/operator")]
pub fn operator(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "operator",
        ItemContext {
            item: Binding::get(&connection)?,
        },
    ))
}

#[post("/save", format = "json", data = "<bindings>")]
pub fn save(connection: Db, bindings: Json<UpdatedBinding>) -> IOracleResult<Redirect> {
    Binding::update(&connection, bindings.into_inner())?;

    Ok(Redirect::to("/operator"))
}

#[get("/csv")]
pub fn csv(connection: Db) -> IOracleResult<String> {
    // hexagrams
    let hs = Hexagram::all(&connection)?;
    if hs.len() == 0 {
        let file_path = "/home/denis/collector/iora/csv/expanded_gua.csv";
        let mut reader = csv::Reader::from_path(file_path)?;
        for result in reader.deserialize() {
            let record: SheetsHexagram = result?;
            let record = UpdatedHexagram {
                binary: record.binary,
                king_wen_order: record.king_wen_order,
                shao_yong_order: record.shao_yong_order,
                gua: record.gua,
                pin_yin: record.pin_yin,
                character: record.character,
                wilheim: record.wilheim,
                huang: record.huang,
                hatcher: record.hatcher,
                no2do: record.no2do,
                inner_ba_gua: record.inner_ba_gua,
                outer_ba_gua: record.outer_ba_gua,
                host_yao: record.host_yao,
                judgment: "".to_string(),
                image: "".to_string(),
                lines: "".to_string(),
            };
            let _h = Hexagram::insert(&connection, record)?;
        }
    }

    //trigrams
    let ts = Trigram::all(&connection)?;
    if ts.len() == 0 {
        let file_path = "/home/denis/collector/iora/csv/trigrams.csv";
        let mut reader = csv::Reader::from_path(file_path)?;
        for result in reader.deserialize() {
            let record: UpdatedTrigram = result?;
            let _t = Trigram::insert(&connection, record)?;
        }
    }

    Ok("Ok".to_string())
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/")
}

#[catch(500)]
pub fn internal_error() -> Redirect {
    Redirect::to("/")
}

#[post("/question", data = "<question>")]
pub fn question(
    config: State<Config>,
    connection: Db,
    question: Form<Question>,
) -> IOracleResult<Redirect> {
    //----------------------------------------------------------------
    println!("send to gate");

    use std::io::prelude::*;
    use std::io::{BufRead, BufReader};
    use std::os::unix::net::{UnixListener, UnixStream};
    if let Ok(mut stream) = UnixStream::connect("/tmp/ioracle.in") {
        if let Err(e) = stream.write_all(b"read") {
            println!("{:?}", e);
        };
    };

    println!("show result");

    use std::fs;
    use std::path::Path;
    let socket = Path::new("/tmp/ioracle.out");
    // Delete old socket if necessary
    if socket.exists() {
        if let Err(error) = std::fs::remove_file("/tmp/ioracle.out") {
            println!("{}", error);
            std::process::exit(1);
        };
    }

    let mut result = "".to_string();
    let listener = UnixListener::bind("/tmp/ioracle.out").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = BufReader::new(stream);
                if let Some(r) = stream.lines().nth(0) {
                    if let Ok(r) = r {
                        result = r;
                    }
                }
                println!("result: {:?}", result);
                break;
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }

    //----------------------------------------------------------------
    // Ok(Redirect::to(format!(
    //     "/answer/{}",
    //     ask(
    //         config,
    //         &connection,
    //         question.email.to_owned(),
    //         question.question.to_owned()
    //     )?
    // )))
    Ok(Redirect::to(format!("/result/{}", result)))
}
