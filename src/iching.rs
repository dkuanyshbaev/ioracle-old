use crate::errors::IOracleResult;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum Line {
    Yang, // solid line
    Yin,  // open line
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Line::Yang => write!(f, "Yang, solid line"),
            Line::Yin => write!(f, "Yin, open line"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Hexagram {
    pub first: Line,
    pub second: Line,
    pub third: Line,
    pub fourth: Line,
    pub fifth: Line,
    pub sixth: Line,
    pub seventh: Line,
    pub eighth: Line,
}

impl Hexagram {
    pub fn new() -> Hexagram {
        Hexagram {
            first: Line::Yang,
            second: Line::Yang,
            third: Line::Yang,
            fourth: Line::Yang,
            fifth: Line::Yang,
            sixth: Line::Yang,
            seventh: Line::Yang,
            eighth: Line::Yang,
        }
    }
}

pub fn ask_iching(hexagram: Hexagram, question: &String) -> IOracleResult<String> {
    println!("iching");

    Ok("".to_string())
}
