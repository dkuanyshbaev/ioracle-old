use crate::models::binding::Binding;
use crate::wires::*;
use rand::distributions::{Distribution, Uniform};
use rs_ws281x::Controller;
use std::fmt;

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
    pub fn random() -> Line {
        let mut rng = rand::thread_rng();
        let line_range = Uniform::from(0..2);
        let line = if line_range.sample(&mut rng) == 0 {
            Line::Yin
        } else {
            Line::Yang
        };

        line
    }

    pub fn render(&self, line_num: i32, controller: &mut Controller, colour: &String) {
        match self {
            Line::Yin => render_yin(line_num, controller, colour),
            Line::Yang => render_yang(line_num, controller, colour),
        }
    }
}

pub struct Trigram {
    pub top: Line,
    pub middle: Line,
    pub bottom: Line,
}

impl fmt::Display for Trigram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => write!(f, "Heaven"),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => write!(f, "Cloud"),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => write!(f, "Sun"),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => write!(f, "Wind"),
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => write!(f, "Thunder"),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => write!(f, "Water"),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => write!(f, "Mountain"),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => write!(f, "Earth"),
        }
    }
}

impl Trigram {
    pub fn render(&self, settings: &Binding, controller: &mut Controller) {
        match self {
            // Heaven
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => {
                pin_on(settings.heaven_pin as u8);
                render_yang(1, controller, &settings.heaven_colour);
                render_yang(2, controller, &settings.heaven_colour);
                render_yang(3, controller, &settings.heaven_colour);
            }
            // Cloud
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => {
                pin_on(settings.cloud_pin as u8);
                render_yin(1, controller, &settings.cloud_colour);
                render_yang(2, controller, &settings.cloud_colour);
                render_yang(3, controller, &settings.cloud_colour);
            }
            // Sun
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => {
                render_yang(1, controller, &settings.sun_colour);
                render_yin(2, controller, &settings.sun_colour);
                render_yang(3, controller, &settings.sun_colour);
            }
            // Wind
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => {
                pin_on(settings.wind_pin as u8);
                render_yin(1, controller, &settings.wind_colour);
                render_yin(2, controller, &settings.wind_colour);
                render_yang(3, controller, &settings.wind_colour);
            }
            // Thunder
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => {
                play_sound("thunder.wav".to_string());
                render_yang(1, controller, &settings.thunder_colour);
                render_yang(2, controller, &settings.thunder_colour);
                render_yin(3, controller, &settings.thunder_colour);
            }
            // Water
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => {
                pin_on(settings.water_pin as u8);
                render_yin(1, controller, &settings.water_colour);
                render_yang(2, controller, &settings.water_colour);
                render_yin(3, controller, &settings.water_colour);
            }
            // Mountain
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => {
                pin_on(settings.mountain_pin as u8);
                play_sound("mountain.wav".to_string());
                render_yang(1, controller, &settings.mountain_colour);
                render_yin(2, controller, &settings.mountain_colour);
                render_yin(3, controller, &settings.mountain_colour);
            }
            // Earth
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => {
                render_yin(1, controller, &"rgb(0, 0, 0)".to_string());
                render_yin(2, controller, &"rgb(0, 0, 0)".to_string());
                render_yin(3, controller, &"rgb(0, 0, 0)".to_string());
            }
        }
    }
}

pub struct Hexagram {
    pub top: Trigram,
    pub bottom: Trigram,
}

impl fmt::Display for Hexagram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "----")
    }
}
