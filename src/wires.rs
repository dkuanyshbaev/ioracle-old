use crate::errors::{IOracleError, IOracleResult};
use crate::iching::{Hexagram, Line, Trigram};
use crate::models::binding::Binding;
use rand::distributions::{Distribution, Uniform};
use rppal::gpio::Gpio;
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
use serialport::prelude::*;
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime};

const LEDS_IN_LINE: i32 = 144;
const LI_SEGMENTS_NUM: i32 = 1;
const YAO_SEGMENTS_NUM: i32 = 6;
const YAO_PIN: i32 = 12;
const LI_PIN: i32 = 13;

pub fn build_controller() -> IOracleResult<Controller> {
    match ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(YAO_PIN)
                .count(YAO_SEGMENTS_NUM * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build(),
        )
        .channel(
            1,
            ChannelBuilder::new()
                .pin(LI_PIN)
                .count(LI_SEGMENTS_NUM * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build(),
        )
        .build()
    {
        Ok(controller) => Ok(controller),
        Err(_) => Err(IOracleError::InternalServerError),
    }
}

pub fn render_yin(line_num: i32, controller: &mut Controller, colour: &String) {
    let leds = controller.leds_mut(0);
    let (a, b, c) = parse_colour(colour);

    let part = LEDS_IN_LINE / 3;
    let position = LEDS_IN_LINE * (line_num - 1);
    for num in position..position + LEDS_IN_LINE {
        if num > position + part && num < position + part * 2 {
            leds[num as usize] = [0, 0, 0, 0];
        } else {
            // leds[num as usize] = [a, b, c, 0];
            leds[num as usize] = [c, a, b, 0];
        }
    }

    if let Err(e) = controller.render() {
        println!("{:?}", e);
    };
}

pub fn render_yang(line_num: i32, controller: &mut Controller, colour: &String) {
    let leds = controller.leds_mut(0);
    let (a, b, c) = parse_colour(colour);

    let position = LEDS_IN_LINE * (line_num - 1);
    for num in position..position + LEDS_IN_LINE {
        // leds[num as usize] = [a, b, c, 0];
        leds[num as usize] = [c, a, b, 0];
    }

    if let Err(e) = controller.render() {
        println!("{:?}", e);
    };
}

pub fn render_fire(controller: &mut Controller) {
    let mut rng1 = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    let start = SystemTime::now();

    loop {
        if let Ok(d) = start.elapsed() {
            if d > Duration::from_secs(5) {
                break;
            };
        }

        let li = controller.leds_mut(1);
        let red_range = Uniform::from(54..255);

        let mut k;
        for i in 0..li.len() - 1 {
            k = i * 9;
            // !!!???
            if k > li.len() - 9 {
                k = li.len() - 9;
            }
            for j in k..k + 9 {
                let r = red_range.sample(&mut rng1);
                let green_range = Uniform::from(0..r / 4);
                let g = green_range.sample(&mut rng2);
                li[j as usize] = [0, g, r, 0];
            }
        }

        std::thread::sleep(Duration::from_millis(70));

        if let Err(e) = controller.render() {
            println!("Fire error: {:?}", e);
        };
    }
}

pub fn pin_on(pin: u8) {
    println!("--------> pin {}: on", pin);
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_high();
        }
    }
}

pub fn pin_off(pin: u8) {
    println!("--------> pin {}: off", pin);
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_low();
        }
    }
}

pub fn colour_on(colour: String, code: String) {
    println!("--------> element colour: {}", colour);

    let full_code = format!("{}{}", code, code);
    if let Ok(mut controller) = build_controller() {
        for i in 1..7 {
            let ch = full_code.chars().nth(i - 1).unwrap();
            if ch == '1' {
                render_yang(i as i32, &mut controller, &colour);
            } else {
                render_yin(i as i32, &mut controller, &colour);
            }
        }
        if code == "101" {
            println!("fire!!!");
            render_fire(&mut controller);
        }
    };
}

pub fn colour_off() {
    println!("--------> element colour off");

    let colour = "rgb(0, 0, 0)".to_string();
    if let Ok(mut controller) = build_controller() {
        for i in 1..7 {
            render_yang(i, &mut controller, &colour);
        }
    };
}

pub fn play_sound(file_name: String) {
    println!("--------> play: {}", file_name);

    let command = format!("omxplayer -o local --no-keys ./sounds/{} &", file_name);
    if let Ok(output) = Command::new(command).output() {
        if !output.status.success() {
            println!("exectution error");
        } else {
            println!("all good");
        }
    }
}

fn parse_colour(colour: &String) -> (u8, u8, u8) {
    let mut str_buff = colour.clone();
    let mut rgb = (255, 255, 255);

    // colour string format:  "rgb(108, 73, 211)"
    let mut str_buff: String = str_buff.drain(4..).collect();
    str_buff.pop();
    let str_parts = str_buff.split(", ");
    let parts: Vec<&str> = str_parts.collect();

    if let Ok(part) = parts[0].parse::<u8>() {
        rgb.0 = part;
    }
    if let Ok(part) = parts[1].parse::<u8>() {
        rgb.1 = part;
    }
    if let Ok(part) = parts[2].parse::<u8>() {
        rgb.2 = part;
    }

    rgb
}

pub fn reset(settings: Binding) {
    println!("Reset");
    if let Ok(mut controller) = build_controller() {
        reset_all(&settings, &mut controller);
    };
}

pub fn reset_all(settings: &Binding, controller: &mut Controller) {
    println!("--------> reset all");

    // all pins off
    pin_off(settings.heaven_pin as u8);
    pin_off(settings.cloud_pin as u8);
    pin_off(settings.wind_pin as u8);
    pin_off(settings.water_pin as u8);
    pin_off(settings.mountain_pin as u8);

    // all leds to resting_colour
    let (a, b, c) = parse_colour(&settings.resting_colour);
    let yao_leds = controller.leds_mut(0);
    // for num in 0..yao_leds.len() - 1 {
    for num in 0..yao_leds.len() {
        yao_leds[num as usize] = [c, a, b, 0];
    }
    let li_leds = controller.leds_mut(1);
    // for num in 0..li_leds.len() - 1 {
    for num in 0..li_leds.len() {
        li_leds[num as usize] = [c, a, b, 0];
    }

    match controller.render() {
        Ok(_) => println!("reset"),
        Err(e) => println!("{:?}", e),
    };
}

pub fn reset_pins(settings: &Binding) {
    println!("--------> reset pins");

    // all pins off
    pin_off(settings.heaven_pin as u8);
    pin_off(settings.cloud_pin as u8);
    pin_off(settings.wind_pin as u8);
    pin_off(settings.water_pin as u8);
    pin_off(settings.mountain_pin as u8);
}

pub fn run_simulation(settings: Binding) -> IOracleResult<()> {
    println!("Simulation");
    let mut controller = build_controller()?;
    thread::sleep(Duration::from_secs(3));

    let line1 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 1: {}", line1);
    line1.render(1, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line2 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 2: {}", line2);
    line2.render(2, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line3 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 3: {}", line3);
    line3.render(3, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let first_trigram = Trigram {
        top: line1,
        middle: line2,
        bottom: line3,
    };
    println!("first_trigram: {}", first_trigram);
    first_trigram.render_first(&settings, &mut controller);

    let line_related1 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let line_related2 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let line_related3 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let first_related = Trigram {
        top: match line_related1 {
            Line::Yin => match first_trigram.top {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match first_trigram.top {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
        middle: match line_related2 {
            Line::Yin => match first_trigram.middle {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match first_trigram.middle {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
        bottom: match line_related3 {
            Line::Yin => match first_trigram.bottom {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match first_trigram.bottom {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
    };
    println!("first_related: {}", first_related);
    thread::sleep(Duration::from_secs(1));

    let line4 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 4: {}", line4);
    line4.render(4, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line5 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 5: {}", line5);
    line5.render(5, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line6 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 6: {}", line6);
    line6.render(6, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let second_trigram = Trigram {
        top: line4,
        middle: line5,
        bottom: line6,
    };
    println!("second_trigram: {}", second_trigram);
    second_trigram.render_second(&settings, &mut controller);

    let line_related4 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let line_related5 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let line_related6 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let second_related = Trigram {
        top: match line_related4 {
            Line::Yin => match second_trigram.top {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match second_trigram.top {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
        middle: match line_related5 {
            Line::Yin => match second_trigram.middle {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match second_trigram.middle {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
        bottom: match line_related6 {
            Line::Yin => match second_trigram.bottom {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match second_trigram.bottom {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
    };
    println!("second_related: {}", second_related);
    thread::sleep(Duration::from_secs(1));

    let hexagram = Hexagram {
        top: second_trigram,
        bottom: first_trigram,
    };
    let related = Hexagram {
        top: second_related,
        bottom: first_related,
    };
    reset_all(&settings, &mut controller);

    println!("hexagram: {:?}", to_binary(&hexagram));
    println!("related: {:?}", to_binary(&related));

    Ok(())
}

pub fn reading(settings: Binding) -> IOracleResult<(Hexagram, Hexagram)> {
    println!("New reading.");
    let mut controller = build_controller()?;
    thread::sleep(Duration::from_secs(3));

    let line1 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 1: {}", line1);
    line1.render(1, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(3));

    let line2 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 2: {}", line2);
    line2.render(2, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(3));

    let line3 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 3: {}", line3);
    line3.render(3, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(3));

    let first_trigram = Trigram {
        top: line1,
        middle: line2,
        bottom: line3,
    };
    println!("first_trigram: {}", first_trigram);
    first_trigram.render_first(&settings, &mut controller);

    let line_related1 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let line_related2 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let line_related3 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let first_related = Trigram {
        top: match line_related1 {
            Line::Yin => match first_trigram.top {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match first_trigram.top {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
        middle: match line_related2 {
            Line::Yin => match first_trigram.middle {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match first_trigram.middle {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
        bottom: match line_related3 {
            Line::Yin => match first_trigram.bottom {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match first_trigram.bottom {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
    };
    println!("first_related: {}", first_related);
    reset_pins(&settings);

    let line4 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 4: {}", line4);
    line4.render(4, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(3));

    let line5 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 5: {}", line5);
    line5.render(5, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(3));

    let line6 = Line::read(
        2,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    println!("Line 6: {}", line6);
    line6.render(6, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(3));

    let second_trigram = Trigram {
        top: line4,
        middle: line5,
        bottom: line6,
    };
    println!("second_trigram: {}", second_trigram);
    second_trigram.render_second(&settings, &mut controller);

    let line_related4 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let line_related5 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let line_related6 = Line::read(
        1,
        settings.multiply.clone(),
        settings.bias.clone(),
        settings.threshold.clone(),
    );
    let second_related = Trigram {
        top: match line_related4 {
            Line::Yin => match second_trigram.top {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match second_trigram.top {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
        middle: match line_related5 {
            Line::Yin => match second_trigram.middle {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match second_trigram.middle {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
        bottom: match line_related6 {
            Line::Yin => match second_trigram.bottom {
                Line::Yin => Line::Yang,
                Line::Yang => Line::Yin,
            },
            Line::Yang => match second_trigram.bottom {
                Line::Yang => Line::Yin,
                Line::Yin => Line::Yang,
            },
        },
    };
    println!("second_related: {}", second_related);

    let hexagram = Hexagram {
        top: second_trigram,
        bottom: first_trigram,
    };
    let related = Hexagram {
        top: second_related,
        bottom: first_related,
    };
    // reset_all(&settings, &mut controller);
    reset_pins(&settings);

    // let hex_binary = to_binary(&hexagram);
    // let rel_binary = to_binary(&related);

    // keep result on LED
    // let h = hex_binary.clone();
    // let r = rel_binary.clone();
    // std::thread::spawn(move || {
    //     show_result(h, r, settings);
    // });

    Ok((hexagram, related))
}

// pub fn show_result(h: String, _r: String, settings: Binding) {
//     println!("{}", h);
//     if let Ok(mut controller) = build_controller() {
//         let mut n = 1;
//         for i in h.chars() {
//             match i {
//                 '1' => render_yang(n, &mut controller, &settings.default_colour),
//                 _ => render_yin(n, &mut controller, &settings.default_colour),
//             }
//             n += 1;
//             println!("{}", i);
//         }
//         thread::sleep(Duration::from_secs(120));
//     };
// }

pub fn to_binary(h: &Hexagram) -> String {
    let mut r = "".to_string();

    match h.top.top {
        Line::Yang => r = format!("{}1", r),
        Line::Yin => r = format!("{}0", r),
    }

    match h.top.middle {
        Line::Yang => r = format!("{}1", r),
        Line::Yin => r = format!("{}0", r),
    }

    match h.top.bottom {
        Line::Yang => r = format!("{}1", r),
        Line::Yin => r = format!("{}0", r),
    }

    match h.bottom.top {
        Line::Yang => r = format!("{}1", r),
        Line::Yin => r = format!("{}0", r),
    }

    match h.bottom.middle {
        Line::Yang => r = format!("{}1", r),
        Line::Yin => r = format!("{}0", r),
    }

    match h.bottom.bottom {
        Line::Yang => r = format!("{}1", r),
        Line::Yin => r = format!("{}0", r),
    }

    r
}

pub fn read_the_pip(delta: u64) -> Vec<i32> {
    let s = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(1),
    };

    let mut data: Vec<i32> = vec![];
    if let Ok(mut port) = serialport::open_with_settings("/dev/ttyACM0", &s) {
        let mut serial_buf: Vec<u8> = vec![0; 512];
        let start = SystemTime::now();
        loop {
            if let Ok(d) = start.elapsed() {
                if d > Duration::from_secs(delta) {
                    break;
                };
            }
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    // println!("Pip val: {}", get_val(&serial_buf[..t]));
                    data.push(get_val(&serial_buf[..t]));
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }

    data
}

fn get_val(buf: &[u8]) -> i32 {
    let mut output = 0;
    let serial_data = std::str::from_utf8(buf).unwrap();
    if let Some(i) = serial_data.find("PiPVal: ") {
        let s = &serial_data[i + 8..];
        if let Some(j) = s.find("\r") {
            let str_value = &s[..j];
            if let Ok(value) = str_value.parse::<i32>() {
                output = value;
            }
        }
    }

    output
}
