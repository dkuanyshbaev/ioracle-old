use rocket_contrib::databases::rusqlite::{params, Connection};
use uuid::Uuid;

// #[derive(Serialize, Debug)]
// pub struct Answer {
//     answer: String,
// }
// -----------------------------------------------------------------------------
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;
use rocket_contrib::databases::rusqlite::Error as RusqliteError;
use std::convert::From;
use std::{error, fmt};

#[derive(Debug)]
pub enum IOracleError {
    NotFound,
    InternalServerError,
}

impl From<RusqliteError> for IOracleError {
    fn from(error: RusqliteError) -> Self {
        match error {
            RusqliteError::QueryReturnedNoRows => IOracleError::NotFound,
            _ => IOracleError::InternalServerError,
        }
    }
}

impl fmt::Display for IOracleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IOracleError::NotFound => write!(f, "NotFound"),
            IOracleError::InternalServerError => write!(f, "InternalServerError"),
        }
    }
}

impl error::Error for IOracleError {
    fn description(&self) -> &str {
        match *self {
            IOracleError::NotFound => "Record not found",
            IOracleError::InternalServerError => "Internal server error",
        }
    }
}

impl<'r> Responder<'r, 'static> for IOracleError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            IOracleError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}
// -----------------------------------------------------------------------------

// pub fn ask(connection: &Connection, email: String, question: String) -> IOracleResult<String> {
pub fn ask(
    connection: &Connection,
    email: String,
    question: String,
) -> Result<String, IOracleError> {
    let answer = ioracle(&question);

    let answer_uuid = save(connection, &email, &question, &answer)?;

    send(&email, &question, &answer);

    // match save(connection, email, question, answer) {
    //     Some(answer_uuid) => {
    //         // send_mail();
    //         answer_uuid
    //     }
    //     None => "".to_string(),
    // }

    Ok(answer_uuid)
}

pub fn ioracle(_question: &String) -> String {
    "the answer".to_string()
}

pub fn save(
    connection: &Connection,
    email: &String,
    question: &String,
    answer: &String,
) -> Result<String, IOracleError> {
    let uuid = Uuid::new_v4();
    let uuid = uuid.to_string();

    connection.execute(
        "create table if not exists answers (
            id integer primary key,
            uuid text not null,
            email text not null,
            question text not null,
            answer text not null,
            created_at datetime default current_timestamp
        )",
        params![],
    )?;

    connection.execute(
        "insert into answers (uuid, email, question, answer) values (?1, ?2, ?3, ?4)",
        params![uuid, email, question, answer],
    )?;

    Ok(uuid)
}

pub fn get_answer(_connection: &Connection, _uuid: String) -> Result<String, IOracleError> {
    // pub fn get_answer(connection: &Connection, uuid: String) -> Option<Answer> {
    // let mut stmt = connection
    //     .prepare("select answer from answers where uuid = ?1")
    //     .unwrap();
    //
    // let answers_iter = stmt
    //     .query_map(&[&uuid], |row| Answer { answer: row.get(0) })
    //     .unwrap();
    //
    // for an in answers_iter {
    //     println!("Found answer {:?}", an.unwrap());
    // }
    // Some(Answer {
    //     answer: "the answer".to_string(),
    // })
    // ---------------------------------------------------------------------
    // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    // let person_iter = stmt.query_map(params![], |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;
    //
    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }

    Ok("the answer".to_string())
}

pub fn send(_email: &String, _question: &String, _answer: &String) {
    println!("sending email...");
}
