use crate::errors::IOracleResult;
use crate::oracle::iching::ask_iching;
use crate::oracle::wires::ask_wires;
use crate::Config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::State;
use rocket_contrib::databases::rusqlite::{params, Connection};
use uuid::Uuid;

pub fn ask_question(
    _config: State<Config>,
    connection: &Connection,
    email: String,
    question: String,
) -> IOracleResult<String> {
    let answer = ioracle(&question)?;
    let answer_uuid = save(connection, &email, &question, &answer)?;
    // send(config, &email, &question, &answer)?;

    Ok(answer_uuid)
}

pub fn ioracle(question: &String) -> IOracleResult<String> {
    let hexagram = ask_wires()?;
    let answer = ask_iching(hexagram, question)?;

    Ok(answer)
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
        "insert into answers (uuid, email, question, answer) values (?1, ?2, ?3, ?4)",
        params![uuid, email, question, answer],
    )?;

    Ok(uuid)
}

pub fn get_answer(connection: &Connection, uuid: String) -> IOracleResult<String> {
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

pub fn send(
    config: State<Config>,
    email: &String,
    _question: &String,
    answer: &String,
) -> IOracleResult<()> {
    let body_text = format!("Your answer: {}", answer);

    let email = Message::builder()
        .from(config.email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject(config.subject.to_owned())
        .body(body_text)
        .unwrap();

    let creds = Credentials::new(config.username.to_owned(), config.password.to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    //TODO: return result
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }

    Ok(())
}
