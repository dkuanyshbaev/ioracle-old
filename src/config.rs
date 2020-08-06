use std::env;

pub struct Config {
    pub email: String,
    pub subject: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new() -> Result<Config, env::VarError> {
        let email = env::var("EMAIL")?;
        let subject = env::var("SUBJECT")?;
        let username = env::var("USERNAME")?;
        let password = env::var("PASSWORD")?;
        Ok(Config {
            email,
            subject,
            username,
            password,
        })
    }
}
