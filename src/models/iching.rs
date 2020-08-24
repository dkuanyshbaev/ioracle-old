// use super::schema::trigrams;
use crate::errors::IOracleResult;
use rand::distributions::{Distribution, Uniform};
// use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum Line {
    Yin,  // open line
    Yang, // solid line
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Line::Yin => write!(f, "Yin, open line"),
            Line::Yang => write!(f, "Yang, solid line"),
        }
    }
}

impl Line {
    pub fn get_random() -> Line {
        let mut rng = rand::thread_rng();
        let line_range = Uniform::from(0..2);
        if line_range.sample(&mut rng) == 0 {
            Line::Yin
        } else {
            Line::Yang
        }
    }

    pub fn from_string(line: &String) -> Line {
        if *line == "Yin".to_string() {
            Line::Yin
        } else {
            Line::Yang
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Trigram {
    pub top: Line,
    pub middle: Line,
    pub bottom: Line,
}

impl Trigram {
    pub fn react(&self, connection: &SqliteConnection) -> IOracleResult<()> {
        println!("-----------we are reacting!");

        Ok(())
    }

    pub fn name(&self, connection: &SqliteConnection) -> IOracleResult<String> {
        match self {
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => Ok("Heaven".to_string()),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => Ok("Cloud".to_string()),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => Ok("Sun".to_string()),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => Ok("Wind".to_string()),
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => Ok("Thunder".to_string()),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => Ok("Water".to_string()),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => Ok("Mountain".to_string()),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => Ok("Earth".to_string()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Hexagram {
    pub top: Trigram,
    pub bottom: Trigram,
}

impl Hexagram {
    pub fn name(&self, connection: &SqliteConnection) -> IOracleResult<String> {
        Ok("?".to_string())
    }
}
