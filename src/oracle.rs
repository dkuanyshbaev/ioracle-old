use crate::errors::IOracleResult;
use crate::iching::*;
use crate::models::binding::Binding;
use crate::models::record::{Record, UpdatedRecord};
use crate::wires::*;
use crate::Config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::State;
use rocket_contrib::databases::diesel::SqliteConnection;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

pub fn reading(question: &String, settings: Binding) -> IOracleResult<String> {
    println!("New reading.");
    thread::sleep(Duration::from_secs(3));

    let mut controller = build_controller()?;

    let line1 = Line::random();
    println!("Line 1: {}", line1);
    line1.render(1, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line2 = Line::random();
    println!("Line 2: {}", line2);
    line2.render(2, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line3 = Line::random();
    println!("Line 3: {}", line3);
    line3.render(3, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let top_trigram = Trigram {
        top: line1,
        middle: line2,
        bottom: line3,
    };
    println!("Top Trigram: {}", top_trigram);
    top_trigram.render(&settings, &mut controller);
    thread::sleep(Duration::from_secs(1));

    let line4 = Line::random();
    println!("Line 4: {}", line4);
    line4.render(4, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line5 = Line::random();
    println!("Line 5: {}", line5);
    line5.render(5, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line6 = Line::random();
    println!("Line 6: {}", line6);
    line6.render(6, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let bottom_trigram = Trigram {
        top: line4,
        middle: line5,
        bottom: line6,
    };
    println!("Bottom Trigram: {}", bottom_trigram);
    bottom_trigram.render(&settings, &mut controller);
    thread::sleep(Duration::from_secs(1));

    let hexagram = Hexagram {
        top: top_trigram,
        bottom: bottom_trigram,
    };
    println!("Hexagram: {}", hexagram);

    reset_all(&settings, &mut controller);

    Ok(question.to_string())
}

pub fn ask(
    config: State<Config>,
    connection: &SqliteConnection,
    email: String,
    question: String,
) -> IOracleResult<String> {
    let settings = Binding::get(&connection)?;
    let answer = reading(&question, settings)?;
    let answer_uuid = save(connection, &email, &question, &answer)?;
    send(config, &email, &question, &answer)?;

    Ok(answer_uuid)
}

pub fn save(
    connection: &SqliteConnection,
    email: &String,
    question: &String,
    answer: &String,
) -> IOracleResult<String> {
    let uuid = Uuid::new_v4();
    let uuid = uuid.to_string();

    let _r = Record::insert(
        &connection,
        UpdatedRecord {
            uuid: uuid.to_string(),
            email: email.to_string(),
            question: question.to_string(),
            answer: answer.to_string(),
        },
    )?;

    Ok(uuid)
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
        .subject("I Oracle")
        .body(body_text)
        .unwrap();

    let creds = Credentials::new(config.username.to_owned(), config.password.to_owned());

    let mailer = SmtpTransport::relay("mail.privateemail.com")
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
