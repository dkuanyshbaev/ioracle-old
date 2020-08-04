use chrono::Utc;
use rusqlite::Connection;
use uuid::Uuid;

pub fn ask(connection: &Connection, email: String, question: String) -> String {
    let answer = ioracle(&question);
    match save(connection, email, question, answer) {
        Some(answer_uuid) => {
            send_mail();
            answer_uuid
        }
        None => "".to_string(),
    }
}

pub fn ioracle(_question: &String) -> String {
    "the answer".to_string()
}

pub fn save(
    connection: &Connection,
    email: String,
    question: String,
    answer: String,
) -> Option<String> {
    let uuid = Uuid::new_v4();
    let answer_uuid = uuid.to_string();
    let time_now = Utc::now();
    let time_now = time_now.to_string();

    match connection.execute(
        "create table if not exists answers (
                id integer primary key,
                uuid text not null,
                email text not null,
                question text not null,
                answer text not null,
                created_at text not null
                )",
        &[],
    ) {
        Ok(_) => println!("ok"),
        Err(_) => return None,
    }

    match connection.execute(
        "insert into answers (uuid, email, question, answer, created_at)
            values (?1, ?2, ?3, ?4, ?5)",
        &[&answer_uuid, &email, &question, &answer, &time_now],
    ) {
        Ok(_) => println!("ok"),
        Err(_) => return None,
    }

    Some(answer_uuid)
}

pub fn get_answer(connection: &Connection, uuid: String) -> Option<String> {
    let mut stmt = connection
        .prepare("select answer from answers where uuid = ?1")
        .unwrap();

    #[derive(Debug)]
    struct Answer {
        answer: String,
    }

    let answers_iter = stmt
        .query_map(&[&uuid], |row| Answer { answer: row.get(0) })
        .unwrap();

    for an in answers_iter {
        println!("Found answer {:?}", an.unwrap());
    }
    Some("the answer".to_string())
}

pub fn send_mail() {
    println!("sending email...");
}
