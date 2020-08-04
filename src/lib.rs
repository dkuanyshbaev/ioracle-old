use chrono::Utc;
use rusqlite::Connection;
use uuid::Uuid;

// #[derive(Serialize)]
// pub struct Answer {
//     uuid: String,
//     email: String,
//     question: String,
//     answer: String,
// }

pub fn ask(connection: &Connection, email: String, question: String) -> String {
    let answer = ioracle(&question);
    let answer_uuid = save(connection, email, question, answer);
    send_mail();

    answer_uuid
}

pub fn ioracle(_question: &String) -> String {
    println!("asking ioracle...");
    "the answer".to_string()
}

pub fn save(connection: &Connection, email: String, question: String, answer: String) -> String {
    println!("---------------------------------");
    println!("saving to db...");

    let uuid = Uuid::new_v4();
    let answer_uuid = uuid.to_string();
    let time_now = Utc::now();
    let time_now = time_now.to_string();

    //--------------------------------------------------------------
    connection
        .execute(
            "create table if not exists answers (
                id integer primary key,
                uuid text not null,
                email text not null,
                question text not null,
                answer text not null,
                created_at text not null
                )",
            &[],
        )
        .unwrap();

    connection
        .execute(
            "insert into answers (uuid, email, question, answer, created_at)
            values (?1, ?2, ?3, ?4, ?5)",
            &[&answer_uuid, &email, &question, &answer, &time_now],
        )
        .unwrap();

    //--------------------------------------------------------------

    answer_uuid
}

pub fn get_answer(connection: &Connection, uuid: String) -> IOracleResult<String> {
    // let mut stmt = conn
    //     .prepare("SELECT id, name, time_created, data FROM person")
    //     .unwrap();
    //
    // let person_iter = stmt
    //     .query_map(&[], |row| Person {
    //         id: row.get(0),
    //         name: row.get(1),
    //         time_created: row.get(2),
    //         data: row.get(3),
    //     })
    //     .unwrap();
    //
    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }

    "".to_string()
}

pub fn send_mail() {
    println!("---------------------------------");
    println!("sending email...");
    println!("---------------------------------");
}
