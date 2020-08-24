// use crate::errors::IOracleResult;
use crate::models::iching::{Line, Trigram};
// use crate::Db;
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
    Json(Line::get_random())
}

#[post("/element", format = "json", data = "<lines>")]
pub fn element(lines: Json<Lines>) -> Json<String> {
    let trigram = Trigram {
        top: Line::from_string(&lines.first),
        middle: Line::from_string(&lines.second),
        bottom: Line::from_string(&lines.third),
    };

    trigram.react();

    Json(trigram.as_element())
}

#[post("/result", format = "json", data = "<all_lines>")]
pub fn result(all_lines: Json<AllLines>) -> Json<String> {
    println!("{:?}", all_lines);

    Json("?".to_string())
}
