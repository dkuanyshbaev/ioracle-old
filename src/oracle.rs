use crate::errors::IOracleResult;
use crate::iching::Hexagram;
use crate::models::binding::Binding;
use crate::models::hexagram;
use crate::models::record::{Record, UpdatedRecord};
use crate::models::trigram;
use crate::wires::{reading, show_hexagram, to_binary};
use crate::Config;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use rocket::State;
use rocket_contrib::databases::diesel::SqliteConnection;
use uuid::Uuid;
// use lettre::transport::smtp::authentication::Credentials;
// use lettre::{Message, SmtpTransport, Transport};

pub fn ask(
    config: State<Config>,
    connection: &SqliteConnection,
    email: String,
    question: String,
) -> IOracleResult<String> {
    let settings = Binding::get(&connection)?;
    let (hexagram, related) = reading(&settings)?;

    let hex_binary = to_binary(&hexagram);
    let rel_binary = to_binary(&related);

    // let ch_lines = get_changing_lines(&hex_binary, &rel_binary);
    show_hexagram(&settings, &hex_binary, &rel_binary);

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
    send(
        config,
        &email,
        &question,
        &answer,
        &full_h,
        &full_r,
        &hex_binary,
        &rel_binary,
        &connection,
    )?;

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
    h_binary: &String,
    r_binary: &String,
    connection: &SqliteConnection,
) -> IOracleResult<()> {
    // let body_text = format!("I ORACLE: {}", answer);
    // let email = Message::builder()
    //     .from(config.email.parse().unwrap())
    //     .to(email.parse().unwrap())
    //     .subject("I Oracle")
    //     .body(body_text)
    //     .unwrap();
    //
    // let creds = Credentials::new(config.username.to_owned(), config.password.to_owned());
    //
    // let mailer = SmtpTransport::relay("mail.privateemail.com")
    //     .unwrap()
    //     .credentials(creds)
    //     .build();
    //
    // match mailer.send(&email) {
    //     Ok(_) => println!("Email sent successfully!"),
    //     Err(e) => println!("Could not send email: {:?}", e),
    // }

    // let h_binary = record.hexagram.clone();
    // let r_binary = record.related.clone();

    // let hexagram = Hexagram::get_by_binary(&connection, h_binary.clone())?;
    // let related = Hexagram::get_by_binary(&connection, r_binary.clone())?;

    let first_trigram =
        trigram::Trigram::get_by_binary(&connection, &(&h_binary[..3]).to_string())?;
    let second_trigram =
        trigram::Trigram::get_by_binary(&connection, &(&h_binary[3..]).to_string())?;
    let first_related =
        trigram::Trigram::get_by_binary(&connection, &(&r_binary[..3]).to_string())?;
    let second_related =
        trigram::Trigram::get_by_binary(&connection, &(&r_binary[3..]).to_string())?;

    let core_primary_first =
        trigram::Trigram::get_by_binary(&connection, &(&h_binary[1..4]).to_string())?;
    let core_primary_second =
        trigram::Trigram::get_by_binary(&connection, &(&h_binary[2..5]).to_string())?;
    let core_related_first =
        trigram::Trigram::get_by_binary(&connection, &(&r_binary[1..4]).to_string())?;
    let core_related_second =
        trigram::Trigram::get_by_binary(&connection, &(&r_binary[2..5]).to_string())?;

    let core_p_binary = format!("{}{}", &h_binary[1..4], &h_binary[2..5]);
    let core_r_binary = format!("{}{}", &r_binary[1..4], &r_binary[2..5]);
    let core_primary = hexagram::Hexagram::get_by_binary(&connection, core_p_binary)?;
    let core_related = hexagram::Hexagram::get_by_binary(&connection, core_r_binary)?;

    let head_text =
        "<h1>I ORACLE<br>HYBRID I CHING DIVINATION SYSTEM<br>PERSONAL READING INTERFACE RESULT:</h1>"
            .to_string();

    let hexagram_text = format!(
        "
        <h4>PRIMARY:</h4>
        <p>{}</p>
        <p>{}</p>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <p>over</p>
        <p>{}</p>
        <p>{}</p>
        ",
        hexagram.king_wen_order,
        hexagram.hatcher,
        hexagram.binary,
        second_trigram.image,
        second_trigram.name,
        first_trigram.image,
        first_trigram.name,
        hexagram.gua,
        hexagram.pin_yin,
        hexagram.character,
        second_trigram.name,
        first_trigram.name,
        hexagram.shao_yong_order,
    );

    let related_text = format!(
        "
        <h4>RELATED:</h4>
        <p>{}</p>
        <p>{}</p>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <p>over</p>
        <p>{}</p>
        <p>{}</p>
        ",
        related.king_wen_order,
        related.hatcher,
        related.binary,
        second_related.image,
        second_related.name,
        first_related.image,
        first_related.name,
        related.gua,
        related.pin_yin,
        related.character,
        second_related.name,
        first_related.name,
        related.shao_yong_order,
    );

    let core_primary_text = format!(
        "
        <h4>CORE PRIMARY:</h4>
        <p>{}</p>
        <p>{}</p>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <p>over</p>
        <p>{}</p>
        <p>{}</p>
        ",
        core_primary.king_wen_order,
        core_primary.hatcher,
        core_primary.binary,
        core_primary_second.image,
        core_primary_second.name,
        core_primary_first.image,
        core_primary_first.name,
        core_primary.gua,
        core_primary.pin_yin,
        core_primary.character,
        core_primary_second.name,
        core_primary_first.name,
        core_primary.shao_yong_order,
    );

    let core_related_text = format!(
        "
        <h4>CORE RELATED:</h4>
        <p>{}</p>
        <p>{}</p>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <h1>{}</h1>
        <p>{}</p>
        <p>over</p>
        <p>{}</p>
        <p>{}</p>
        ",
        core_related.king_wen_order,
        core_related.hatcher,
        core_related.binary,
        core_related_second.image,
        core_related_second.name,
        core_related_first.image,
        core_related_first.name,
        core_related.gua,
        core_related.pin_yin,
        core_related.character,
        core_related_second.name,
        core_related_first.name,
        core_related.shao_yong_order,
    );

    let traditional_text = format!(
        "
        <h4>TRADITIONAL INTERPRETATIONS:</h4>
        <br>
        <h4>King Wen's Decision</h4>
        <br>
        {}
        <br>
        <br>
        <h4>The Image</h4>
        <br>
        {}
        <br>
        <br>
        <h4>Duke of Zhou</h4>
        <br>
        {}
        <br>
        <br>
        ",
        hexagram.judgment, hexagram.image, hexagram.lines,
    );

    let traditional_related_text = format!(
        "
        <h4>King Wen's Decision Related</h4>
        <br>
        {}
        <br>
        <br>
        <h4>The Image Related</h4>
        <br>
        {}
        <br>
        <br>
        <h4>Duke of Zhou Related</h4>
        <br>
        {}
        ",
        related.judgment, related.image, related.lines,
    );

    let body_text = format!(
        "{}<hr>{}<br>{}<br>{}<br>{}<hr><p>{}<p>{}{}",
        head_text,
        hexagram_text,
        related_text,
        core_primary_text,
        core_related_text,
        answer,
        traditional_text,
        traditional_related_text,
    );

    match EmailBuilder::new()
        .from(config.email.parse::<String>().unwrap())
        .to(email.parse::<String>().unwrap())
        .subject("I ORACLE")
        .html(body_text)
        .build()
    {
        Ok(email) => {
            let credentials =
                (config.username.to_owned(), config.password.to_owned()).into_credentials();

            let mut client = SmtpClient::new_simple("mail.privateemail.com")
                .unwrap()
                .credentials(credentials)
                .transport();

            let _result = client.send(email.into());
            println!("Email sent successfully!");
        }
        Err(error) => println!("Email sending error: {:?}", error),
    }

    Ok(())
}

pub fn generate(question: String, _hexagram: Hexagram) -> IOracleResult<String> {
    Ok(question.to_string())
}

pub fn get_changing_lines(h: &String, r: &String) -> String {
    let mut result = "".to_string();

    for i in 0..6 {
        if h.chars().nth(i) == r.chars().nth(i) {
            result = format!("{}1", result);
        } else {
            result = format!("{}0", result);
        }
    }

    result
}
