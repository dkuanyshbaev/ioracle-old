use rand::distributions::{Distribution, Uniform};
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
    pub fn new() -> Trigram {
        Trigram {
            top: Line::Yin,
            middle: Line::Yin,
            bottom: Line::Yang,
        }
    }

    pub fn react(&self) {
        println!("-----------we are reacting!");
    }

    pub fn as_element(&self) -> String {
        match self {
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => "Heaven".to_string(),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => "Cloud".to_string(),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => "Sun".to_string(),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => "Wind".to_string(),
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => "Thunder".to_string(),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => "Water".to_string(),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => "Mountain".to_string(),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => "Earth".to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Hexagram {
    pub top: Trigram,
    pub bottom: Trigram,
}

impl Hexagram {
    // pub fn new() -> Hexagram {
    //     Hexagram {
    //         top: Trigram::new(),
    //         bottom: Trigram::new(),
    //     }
    // }
}
