use crate::errors::IOracleResult;
use crate::models::iching::{Hexagram, Line, Trigram};
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

#[get("/touch")]
pub fn touch() -> Json<Line> {
    // Json(Line::get_touch())
    Json(Line::get_random())
}

#[post("/element", format = "json", data = "<lines>")]
pub fn element(connection: Db, lines: Json<Lines>) -> IOracleResult<Json<String>> {
    let trigram = Trigram {
        top: Line::from_string(&lines.first),
        middle: Line::from_string(&lines.second),
        bottom: Line::from_string(&lines.third),
    };
    trigram.react(&connection)?;

    Ok(Json(trigram.name(&connection)?))
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
