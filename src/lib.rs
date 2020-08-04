use uuid::Uuid;

struct Record {
    id: i32,
    uuid: String,
    email: String,
    question: String,
    answer: String,
}

pub fn ask(email: String, question: String) -> String {
    let answer = ioracle(&question);
    let answer_uuid = save(email, question, answer);
    send_mail();

    answer_uuid
}

pub fn ioracle(question: &String) -> String {
    println!("question: {}", question);
    "the answer".to_string()
}

pub fn save(email: String, question: String, answer: String) -> String {
    println!("email: {}", email);
    println!("question: {}", question);
    println!("answer: {}", answer);
    println!("saving to db...");

    let uuid = Uuid::new_v4();
    let answer_uuid = uuid.to_string();
    answer_uuid
}

pub fn send_mail() {
    println!("sending email...");
}
