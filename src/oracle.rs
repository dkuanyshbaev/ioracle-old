use crate::errors::IOracleResult;
use crate::iching::Hexagram;
use crate::models::binding::Binding;
use crate::models::hexagram;
use crate::models::record::{Record, UpdatedRecord};
use crate::wires::{reading, to_binary};
use crate::Config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::State;
use rocket_contrib::databases::diesel::SqliteConnection;
use uuid::Uuid;

pub fn ask(
    config: State<Config>,
    connection: &SqliteConnection,
    email: String,
    question: String,
) -> IOracleResult<String> {
    let settings = Binding::get(&connection)?;
    let (hexagram, related) = reading(settings)?;

    let hex_binary = to_binary(&hexagram);
    let rel_binary = to_binary(&related);
    let full_h = hexagram::Hexagram::get_by_binary(connection, hex_binary.clone())?;
    let full_r = hexagram::Hexagram::get_by_binary(connection, rel_binary.clone())?;
    println!("hex: {:#?}", full_h);
    println!("rel: {:#?}", full_r);

    let answer = generate(question.clone(), hexagram)?;
    let answer_uuid = save(
        connection,
        &email,
        &question,
        &answer,
        &hex_binary,
        &rel_binary,
    )?;
    send(config, &email, &question, &answer, &full_h, &full_r)?;

    Ok(answer_uuid)
}

pub fn save(
    connection: &SqliteConnection,
    email: &String,
    question: &String,
    answer: &String,
    hexagram: &String,
    related: &String,
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
            hexagram: hexagram.to_string(),
            related: related.to_string(),
        },
    )?;

    Ok(uuid)
}

pub fn send(
    config: State<Config>,
    email: &String,
    _question: &String,
    answer: &String,
    hexagram: &hexagram::Hexagram,
    related: &hexagram::Hexagram,
) -> IOracleResult<()> {
    let body_text = format!("I ORACLE: {}", answer);

    //             <h1 class="cover-heading">I ORACLE</h1>
    // <h4 id="head1">HYBRID I CHING DIVINATION SYSTEM</h4>
    // <h4 id="head2">PERSONAL READING INTERFACE</h4>

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

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }

    Ok(())
}

pub fn generate(question: String, _hexagram: Hexagram) -> IOracleResult<String> {
    Ok(question.to_string())
}
