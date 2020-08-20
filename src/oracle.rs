use crate::errors::{IOracleError, IOracleResult};
use crate::iching::ask_iching;
use crate::wires::ask_wires;
use crate::Config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::State;
use rocket_contrib::databases::rusqlite::{params, Connection};
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct TrigramRow {
    id: i32,
    name: String,
    image: String,
    description: String,
}

#[derive(FromForm)]
pub struct TrigramEdit {
    name: String,
    description: String,
}

pub fn get_trigrams(connection: &Connection) -> IOracleResult<Vec<TrigramRow>> {
    let mut stmt = connection.prepare("select id, name, image, description from trigrams")?;
    let trigram_iter = stmt.query_map(params![], |row| {
        Ok(TrigramRow {
            id: row.get(0)?,
            name: row.get(1)?,
            image: row.get(2)?,
            description: row.get(3)?,
        })
    })?;

    let mut ts: Vec<TrigramRow> = Vec::new();
    for trigram in trigram_iter {
        if let Ok(t) = trigram {
            ts.push(t);
        }
    }

    Ok(ts)
}

pub fn get_trigram(connection: &Connection, id: i32) -> IOracleResult<TrigramRow> {
    let mut stmt =
        connection.prepare("select id, name, image, description from trigrams where id = ?1")?;
    let trigram_iter = stmt.query_map(params![id], |row| {
        Ok(TrigramRow {
            id: row.get(0)?,
            name: row.get(1)?,
            image: row.get(2)?,
            description: row.get(3)?,
        })
    })?;

    let mut ts: Vec<TrigramRow> = Vec::new();
    for trigram in trigram_iter {
        if let Ok(t) = trigram {
            ts.push(t);
        }
    }

    match ts.pop() {
        Some(t) => Ok(t),
        None => Err(IOracleError::NotFound),
    }
}

pub fn update_tri(connection: &Connection, id: i32, trigram: TrigramEdit) -> IOracleResult<()> {
    connection.execute(
        "update trigrams set name = ?1, description = ?2 where id = ?3",
        params![trigram.name, trigram.description, id],
    )?;

    Ok(())
}

#[derive(Serialize, Debug)]
pub struct HexagramRow {
    id: i32,
    name: String,
    image: String,
    description: String,
}

pub fn get_hexagrams(connection: &Connection) -> IOracleResult<Vec<HexagramRow>> {
    let mut stmt = connection.prepare("select id, name, image, description from hexagrams")?;
    let hexagram_iter = stmt.query_map(params![], |row| {
        Ok(HexagramRow {
            id: row.get(0)?,
            name: row.get(1)?,
            image: row.get(2)?,
            description: row.get(3)?,
        })
    })?;

    let mut hs: Vec<HexagramRow> = Vec::new();
    for hexagram in hexagram_iter {
        if let Ok(h) = hexagram {
            hs.push(h);
        }
    }

    Ok(hs)
}

pub fn ask(
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
