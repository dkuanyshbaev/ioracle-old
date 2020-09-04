use crate::errors::IOracleResult;
use crate::models::binding::Binding;
use crate::oracle::wires::*;
use rand::distributions::{Distribution, Uniform};
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
    pub fn get_touch(line_num: u8) -> Line {
        let mut rng = rand::thread_rng();
        let line_range = Uniform::from(0..2);
        let line = if line_range.sample(&mut rng) == 0 {
            Line::Yin
        } else {
            Line::Yang
        };

        line.on(line_num);

        line
    }

    pub fn on(&self, line_num: u8) {
        match self {
            Line::Yin => yin(line_num),
            Line::Yang => yang(line_num),
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
        let bindings = Binding::get(&connection)?;
        match self {
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => heaven(bindings.heaven_colour, bindings.heaven_pin as u8),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => cloud(bindings.cloud_colour, bindings.cloud_pin as u8),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => sun(bindings.sun_colour, bindings.sun_pin as u8),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => wind(bindings.wind_colour, bindings.wind_pin as u8),
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => thunder(bindings.thunder_colour, bindings.thunder_sound),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => water(bindings.wind_colour, bindings.water_pin as u8),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => mountain(bindings.mountain_colour, bindings.mountain_sound),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => earth(bindings.earth_colour, bindings.earth_pin as u8),
        }

        Ok(())
    }

    pub fn name(&self) -> String {
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
    pub fn name(&self, _connection: &SqliteConnection) -> IOracleResult<String> {
        Ok("?".to_string())
    }
}
