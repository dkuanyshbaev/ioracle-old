use crate::errors::IOracleResult;
use std::fmt;

#[derive(Debug, Serialize)]
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

#[derive(Serialize, Debug)]
pub struct Trigram {
    pub top: Line,
    pub middle: Line,
    pub bottom: Line,
}

impl Trigram {
    pub fn new() -> Trigram {
        Trigram {
            top: Line::Yin,
            middle: Line::Yin,
            bottom: Line::Yang,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Hexagram {
    pub top: Trigram,
    pub bottom: Trigram,
}

impl Hexagram {
    pub fn new() -> Hexagram {
        Hexagram {
            top: Trigram::new(),
            bottom: Trigram::new(),
        }
    }
}

pub fn ask_iching(_hexagram: Hexagram, _question: &String) -> IOracleResult<String> {
    println!("iching");

    Ok("".to_string())
}
