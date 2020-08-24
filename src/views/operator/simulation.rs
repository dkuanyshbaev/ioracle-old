use crate::errors::IOracleResult;
use crate::oracle::iching::{Hexagram, Line, Trigram};
use crate::oracle::wires::*;
use crate::Db;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lines {
    first: String,
    second: String,
    third: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllLines {
    first: String,
    second: String,
    third: String,
    fourth: String,
    fifth: String,
    sixth: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pin: u8,
    colour: String,
    sound: String,
}

#[get("/touch/<line_num>")]
pub fn touch(line_num: u8) -> Json<Line> {
    Json(Line::get_touch(line_num))
}

#[post("/element", format = "json", data = "<lines>")]
pub fn element(connection: Db, lines: Json<Lines>) -> IOracleResult<Json<String>> {
    let trigram = Trigram {
        top: Line::from_string(&lines.first),
        middle: Line::from_string(&lines.second),
        bottom: Line::from_string(&lines.third),
    };
    trigram.react(&connection)?;

    Ok(Json(trigram.name()))
}

#[post("/result", format = "json", data = "<all_lines>")]
pub fn result(connection: Db, all_lines: Json<AllLines>) -> IOracleResult<Json<String>> {
    let top_trigram = Trigram {
        top: Line::from_string(&all_lines.first),
        middle: Line::from_string(&all_lines.second),
        bottom: Line::from_string(&all_lines.third),
    };
    let bottom_trigram = Trigram {
        top: Line::from_string(&all_lines.fourth),
        middle: Line::from_string(&all_lines.fifth),
        bottom: Line::from_string(&all_lines.sixth),
    };
    let hexagram = Hexagram {
        top: top_trigram,
        bottom: bottom_trigram,
    };

    Ok(Json(hexagram.name(&connection)?))
}

#[post("/heaven", format = "json", data = "<test>")]
pub fn heaven_test(test: Json<Test>) -> Json<String> {
    heaven(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/cloud", format = "json", data = "<test>")]
pub fn cloud_test(test: Json<Test>) -> Json<String> {
    cloud(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/sun", format = "json", data = "<test>")]
pub fn sun_test(test: Json<Test>) -> Json<String> {
    sun(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/wind", format = "json", data = "<test>")]
pub fn wind_test(test: Json<Test>) -> Json<String> {
    wind(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/thunder", format = "json", data = "<test>")]
pub fn thunder_test(test: Json<Test>) -> Json<String> {
    thunder(test.colour.to_owned(), test.sound.to_owned());

    Json("ok".to_string())
}

#[post("/water", format = "json", data = "<test>")]
pub fn water_test(test: Json<Test>) -> Json<String> {
    water(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/mountain", format = "json", data = "<test>")]
pub fn mountain_test(test: Json<Test>) -> Json<String> {
    mountain(test.colour.to_owned(), test.sound.to_owned());

    Json("ok".to_string())
}

#[post("/earth", format = "json", data = "<test>")]
pub fn earth_test(test: Json<Test>) -> Json<String> {
    earth(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}
