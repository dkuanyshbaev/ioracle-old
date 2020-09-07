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

#[post("/heaven/on", format = "json", data = "<test>")]
pub fn heaven_test_on(test: Json<Test>) -> Json<String> {
    heaven_on(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/heaven/off", format = "json", data = "<test>")]
pub fn heaven_test_off(test: Json<Test>) -> Json<String> {
    heaven_off(test.pin);

    Json("ok".to_string())
}

#[post("/cloud/on", format = "json", data = "<test>")]
pub fn cloud_test_on(test: Json<Test>) -> Json<String> {
    cloud_on(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/cloud/off", format = "json", data = "<test>")]
pub fn cloud_test_off(test: Json<Test>) -> Json<String> {
    cloud_off(test.pin);

    Json("ok".to_string())
}

#[post("/sun/on", format = "json", data = "<test>")]
pub fn sun_test_on(test: Json<Test>) -> Json<String> {
    sun_on(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/sun/off", format = "json", data = "<test>")]
pub fn sun_test_off(test: Json<Test>) -> Json<String> {
    sun_off(test.pin);

    Json("ok".to_string())
}

#[post("/wind/on", format = "json", data = "<test>")]
pub fn wind_test_on(test: Json<Test>) -> Json<String> {
    wind_on(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/wind/off", format = "json", data = "<test>")]
pub fn wind_test_off(test: Json<Test>) -> Json<String> {
    wind_off(test.pin);

    Json("ok".to_string())
}

#[post("/thunder/on", format = "json", data = "<test>")]
pub fn thunder_test_on(test: Json<Test>) -> Json<String> {
    thunder_on(test.colour.to_owned(), test.sound.to_owned());

    Json("ok".to_string())
}

#[post("/thunder/off", format = "json", data = "<test>")]
pub fn thunder_test_off(test: Json<Test>) -> Json<String> {
    thunder_off(test.sound.to_owned());

    Json("ok".to_string())
}

#[post("/water/on", format = "json", data = "<test>")]
pub fn water_test_on(test: Json<Test>) -> Json<String> {
    water_on(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/water/off", format = "json", data = "<test>")]
pub fn water_test_off(test: Json<Test>) -> Json<String> {
    water_off(test.pin);

    Json("ok".to_string())
}

#[post("/mountain/on", format = "json", data = "<test>")]
pub fn mountain_test_on(test: Json<Test>) -> Json<String> {
    mountain_on(test.colour.to_owned(), test.sound.to_owned());

    Json("ok".to_string())
}

#[post("/mountain/off", format = "json", data = "<test>")]
pub fn mountain_test_off(test: Json<Test>) -> Json<String> {
    mountain_off(test.sound.to_owned());

    Json("ok".to_string())
}

#[post("/earth/on", format = "json", data = "<test>")]
pub fn earth_test_on(test: Json<Test>) -> Json<String> {
    earth_on(test.colour.to_owned(), test.pin);

    Json("ok".to_string())
}

#[post("/earth/off", format = "json", data = "<test>")]
pub fn earth_test_off(test: Json<Test>) -> Json<String> {
    earth_off(test.pin);

    Json("ok".to_string())
}
