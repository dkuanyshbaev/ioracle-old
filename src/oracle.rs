use crate::errors::IOracleResult;
use rocket_contrib::databases::rusqlite::{params, Connection};
use uuid::Uuid;

pub fn ioracle(_question: &String) -> IOracleResult<String> {
    Ok("the answer".to_string())
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

pub fn get(_connection: &Connection, _uuid: String) -> IOracleResult<String> {
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

    // Err(IOracleError::NotFound)
    Ok("the answer".to_string())
}

pub fn send(_email: &String, _question: &String, _answer: &String) -> IOracleResult<()> {
    println!("sending email...");
    Ok(())
}
