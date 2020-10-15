use crate::errors::{IOracleError, IOracleResult};
use crate::models::binding::Binding;
use crate::oracle::iching::{Hexagram, Line, Trigram};
use rand::distributions::{Distribution, Uniform};
use rppal::gpio::Gpio;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::Controller;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime};

const LEDS_IN_LINE: i32 = 144;
const LI_SEGMENTS_NUM: i32 = 1;
const YAO_SEGMENTS_NUM: i32 = 6;

pub fn build_controller() -> IOracleResult<Controller> {
    match ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(YAO_SEGMENTS_NUM * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build(),
        )
        .channel(
            1,
            ChannelBuilder::new()
                .pin(19)
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

    match controller.render() {
        Ok(_) => println!("yin"),
        Err(e) => println!("{:?}", e),
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

    match controller.render() {
        Ok(_) => println!("yang"),
        Err(e) => println!("{:?}", e),
    };
}

pub fn pin_on(pin: u8) {
    println!("--------> pin {}: on", pin);

    //---------------------------------
    // const ADDR_DS3231: u16 = 0x20;
    //
    // if pin == 2 {
    //     use rppal::i2c::I2c;
    //     // if let Ok(mut i2c) = I2c::new() {
    //     if let Ok(mut i2c) = I2c::with_bus(1) {
    //         //--------------------------
    //         if let Ok(res) = i2c.set_slave_address(ADDR_DS3231) {
    //             //--------------------------
    //             if let Ok(r) = i2c.write(&[1]) {
    //                 println!("{:?}", r);
    //                 //--------------------------
    //             };
    //         };
    //     };
    // }

    //---------------------------------

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

pub fn element_on(pin: u8, colour: String, code: String) {
    println!(
        "--------> element pin {}: on, element colour: {}",
        pin, colour
    );

    pin_on(pin);

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
            render_fire(&mut controller);
        }
    };
}

pub fn element_off(pin: u8) {
    println!("--------> element pin {}: off", pin);

    pin_off(pin);

    let colour = "rgb(0, 0, 0)".to_string();
    if let Ok(mut controller) = build_controller() {
        for i in 1..7 {
            render_yang(i, &mut controller, &colour);
        }
    };
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
    for num in 0..yao_leds.len() - 1 {
        yao_leds[num as usize] = [c, a, b, 0];
    }
    let li_leds = controller.leds_mut(1);
    for num in 0..li_leds.len() - 1 {
        li_leds[num as usize] = [c, a, b, 0];
    }

    match controller.render() {
        Ok(_) => println!("reset"),
        Err(e) => println!("{:?}", e),
    };
}

pub fn run_simulation(settings: Binding) -> IOracleResult<()> {
    println!("Simulation");

    let mut controller = build_controller()?;

    let line1 = Line::random();
    println!("Line 1: {}", line1);
    line1.render(1, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line2 = Line::random();
    println!("Line 2: {}", line2);
    line2.render(2, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line3 = Line::random();
    println!("Line 3: {}", line3);
    line3.render(3, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let top_trigram = Trigram {
        top: line1,
        middle: line2,
        bottom: line3,
    };
    println!("Top Trigram: {}", top_trigram);
    top_trigram.render(&settings, &mut controller);
    thread::sleep(Duration::from_secs(1));

    let line4 = Line::random();
    println!("Line 4: {}", line4);
    line4.render(4, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line5 = Line::random();
    println!("Line 5: {}", line5);
    line5.render(5, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line6 = Line::random();
    println!("Line 6: {}", line6);
    line6.render(6, &mut controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let bottom_trigram = Trigram {
        top: line4,
        middle: line5,
        bottom: line6,
    };
    println!("Bottom Trigram: {}", bottom_trigram);
    bottom_trigram.render(&settings, &mut controller);
    thread::sleep(Duration::from_secs(1));

    let hexagram = Hexagram {
        top: top_trigram,
        bottom: bottom_trigram,
    };
    println!("Hexagram: {}", hexagram);

    reset_all(&settings, &mut controller);

    Ok(())
}
