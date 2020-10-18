use crate::models::binding::Binding;
use crate::wires::*;
// use rand::distributions::{Distribution, Uniform};
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
    // pub fn random() -> Line {
    //     let mut rng = rand::thread_rng();
    //     let line_range = Uniform::from(0..2);
    //     let line = if line_range.sample(&mut rng) == 0 {
    //         Line::Yin
    //     } else {
    //         Line::Yang
    //     };
    //
    //     line
    // }

    pub fn read(delta: u64, m: String, b: String, t: String) -> Line {
        let _m: f32 = m.parse().unwrap_or_else(|_| 1.0);
        let b: f32 = b.parse().unwrap_or_else(|_| 0.0);
        let t: f32 = t.parse().unwrap_or_else(|_| 0.0);

        let data = read_the_pip(delta);
        println!("data: {:?}", data);

        let mut min = 0;
        if let Some(m) = data.iter().min() {
            min = *m;
        };
        println!("min: {}", min);

        let mut max = 0;
        if let Some(m) = data.iter().max() {
            max = *m;
        };
        println!("max: {}", max);

        let n_data = data.iter().map(|&i| i as f32 - b).collect::<Vec<f32>>();
        println!("n_data = {:?}", n_data);

        let mut mins: Vec<f32> = vec![];
        let mut maxs: Vec<f32> = vec![];
        for i in n_data.windows(3) {
            if i[1] > i[0] && i[1] > i[2] && i[1] > t {
                // println!("local max extremum = {:?}", i[1]);
                maxs.push(i[1]);
            }
            if i[1] < i[0] && i[1] < i[2] && i[1].abs() > t {
                // println!("local min extremum = {:?}", i[1]);
                mins.push(i[1]);
            }
            // println!("windows iter = {:?}", i);
        }

        println!("mins = {:?}", mins);
        // println!("mins len = {:?}", mins.len());
        println!("maxs = {:?}", maxs);
        // println!("maxs len = {:?}", maxs.len());

        if maxs.len() > mins.len() {
            Line::Yang
        } else {
            Line::Yin
        }
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
    pub fn render_first(&self, settings: &Binding, controller: &mut Controller) {
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
                render_fire(controller);
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

    pub fn render_second(&self, settings: &Binding, controller: &mut Controller) {
        match self {
            // Heaven
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => {
                pin_on(settings.heaven_pin as u8);
                render_yang(4, controller, &settings.heaven_colour);
                render_yang(5, controller, &settings.heaven_colour);
                render_yang(6, controller, &settings.heaven_colour);
            }
            // Cloud
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => {
                pin_on(settings.cloud_pin as u8);
                render_yin(4, controller, &settings.cloud_colour);
                render_yang(5, controller, &settings.cloud_colour);
                render_yang(6, controller, &settings.cloud_colour);
            }
            // Sun
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => {
                render_yang(4, controller, &settings.sun_colour);
                render_yin(5, controller, &settings.sun_colour);
                render_yang(6, controller, &settings.sun_colour);
            }
            // Wind
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => {
                pin_on(settings.wind_pin as u8);
                render_yin(4, controller, &settings.wind_colour);
                render_yin(5, controller, &settings.wind_colour);
                render_yang(6, controller, &settings.wind_colour);
            }
            // Thunder
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => {
                play_sound("thunder.wav".to_string());
                render_yang(4, controller, &settings.thunder_colour);
                render_yang(5, controller, &settings.thunder_colour);
                render_yin(6, controller, &settings.thunder_colour);
            }
            // Water
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => {
                pin_on(settings.water_pin as u8);
                render_yin(4, controller, &settings.water_colour);
                render_yang(5, controller, &settings.water_colour);
                render_yin(6, controller, &settings.water_colour);
            }
            // Mountain
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => {
                pin_on(settings.mountain_pin as u8);
                play_sound("mountain.wav".to_string());
                render_yang(4, controller, &settings.mountain_colour);
                render_yin(5, controller, &settings.mountain_colour);
                render_yin(6, controller, &settings.mountain_colour);
            }
            // Earth
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => {
                render_yin(4, controller, &"rgb(0, 0, 0)".to_string());
                render_yin(5, controller, &"rgb(0, 0, 0)".to_string());
                render_yin(6, controller, &"rgb(0, 0, 0)".to_string());
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
