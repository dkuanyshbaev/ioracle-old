use crate::config::Config;
use crate::errors::IOracleResult;
use crate::oracle::{ask, get, get_hexagrams, get_trigram, get_trigrams, update_tri, TrigramEdit};
use crate::Db;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct Question {
    email: String,
    question: String,
}

#[derive(Serialize, Debug)]
struct Answer {
    answer: String,
}

#[derive(Serialize)]
struct NoContext {}

#[derive(Serialize)]
struct ListContext<T> {
    items: Vec<T>,
}

#[derive(Serialize)]
struct ItemContext<T> {
    item: T,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[post("/question", data = "<question>")]
pub fn question(
    config: State<Config>,
    connection: Db,
    question: Option<Form<Question>>,
) -> IOracleResult<Redirect> {
    match question {
        Some(q) => Ok(Redirect::to(format!(
            "/answer/{}",
            ask(
                config,
                &connection,
                q.email.to_owned(),
                q.question.to_owned()
            )?
        ))),
        None => Ok(Redirect::to("/")),
    }
}

#[get("/answer/<uuid>")]
pub fn answer(connection: Db, uuid: String) -> IOracleResult<Template> {
    Ok(Template::render(
        "answer",
        Answer {
            answer: get(&connection, uuid)?,
        },
    ))
}

#[get("/operator")]
pub fn operator() -> Template {
    Template::render("operator", NoContext {})
}

#[get("/run")]
pub fn run() -> Template {
    Template::render("run", NoContext {})
}

#[get("/trigrams")]
pub fn trigrams(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "trigrams",
        ListContext {
            items: get_trigrams(&connection)?,
        },
    ))
}

#[get("/edit/trigram/<id>")]
pub fn edit_trigram(connection: Db, id: i32) -> IOracleResult<Template> {
    Ok(Template::render(
        "edit_trigram",
        ItemContext {
            item: get_trigram(&connection, id)?,
        },
    ))
}

#[post("/edit/trigram/<id>", data = "<trigram>")]
pub fn update_trigram(
    connection: Db,
    id: i32,
    trigram: Form<TrigramEdit>,
) -> IOracleResult<Redirect> {
    let t = trigram.into_inner();
    update_tri(&connection, id, t)?;

    Ok(Redirect::to("/trigrams"))
    // match trigram {
    //     Some(t) => Ok(update_tri(&connection, id, t)?),
    //     None => Ok(Redirect::to("/trigrams")),
    // }
}

#[get("/hexagrams")]
pub fn hexagrams(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "hexagrams",
        ListContext {
            items: get_hexagrams(&connection)?,
        },
    ))
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", NoContext {})
}

#[catch(500)]
pub fn internal_error() -> Template {
    Template::render("500", NoContext {})
}
