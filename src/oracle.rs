use crate::errors::IOracleResult;
use rocket_contrib::databases::rusqlite::{params, Connection};
use uuid::Uuid;

pub fn ask(connection: &Connection, email: String, question: String) -> IOracleResult<String> {
    let answer = ioracle(&question)?;
    let answer_uuid = save(connection, &email, &question, &answer)?;
    send(&email, &question, &answer)?;

    Ok(answer_uuid)
}

pub fn ioracle(_question: &String) -> IOracleResult<String> {
    Ok("the answer!".to_string())
}

pub fn save(
    connection: &Connection,
    email: &String,
    question: &String,
    answer: &String,
) -> IOracleResult<String> {
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

pub fn get(connection: &Connection, uuid: String) -> IOracleResult<String> {
    let mut stmt = connection.prepare("select answer from answers where uuid = ?1")?;
    let answers_iter = stmt.query_map(params![uuid], |row| Ok(row.get(0)?))?;

    let mut answer = "".to_string();
    for ai in answers_iter {
        if let Ok(a) = ai {
            answer = a;
        }
    }

    Ok(answer)
}

pub fn send(_email: &String, _question: &String, _answer: &String) -> IOracleResult<()> {
    Ok(())
}
