use crate::errors::{IOracleError, IOracleResult};
use crate::models::binding::Binding;
use crate::oracle::iching::{Hexagram, Line, Trigram};
use rppal::gpio::Gpio;
use rppal::pwm::Pwm;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::Controller;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;
use std::process::Command;
use std::thread;
use std::time::Duration;

const LEDS_IN_LINE: i32 = 144;

pub fn build_controller() -> IOracleResult<Controller> {
    match ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(6 * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build(),
        )
        .channel(
            1,
            ChannelBuilder::new()
                .pin(19)
                .count(1 * LEDS_IN_LINE)
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
            leds[num as usize] = [a, b, c, 0];
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
        leds[num as usize] = [a, b, c, 0];
    }

    match controller.render() {
        Ok(_) => println!("yang"),
        Err(e) => println!("{:?}", e),
    };
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

pub fn element_on(pin: u8, colour: String) {
    println!(
        "--------> element pin {}: on, element colour: {}",
        pin, colour
    );

    pin_on(pin);

    if let Ok(mut controller) = build_controller() {
        for i in 0..6 {
            render_yang(i, &mut controller, &colour);
        }
    };
}

pub fn element_off(pin: u8) {
    println!("--------> element pin {}: off", pin);

    pin_off(pin);
    let colour = "rgb(0, 0, 0)".to_string();

    if let Ok(mut controller) = build_controller() {
        for i in 0..6 {
            render_yang(i, &mut controller, &colour);
        }
    };
}

pub fn set_pwm(pwm: &Pwm, pin: i32, freq: i32, cycles: String) {
    println!(
        ">>>> set pwm pin: {}, freq: {}, cycles: {}",
        pin, freq, cycles
    );

    // Reconfigure the PWM channel for an 8 Hz frequency, 50% duty cycle.
    // pwm.set_frequency(8.0, 0.5)?;

    if let Err(e) = pwm.set_frequency(freq as f64, 0.5) {
        println!("Can't set pwm frequency: {}", e);
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
    pin_off(settings.sun_pin as u8);
    pin_off(settings.wind_pin as u8);
    pin_off(settings.water_pin as u8);
    pin_off(settings.mountain_pin as u8);

    // all leds to resting_colour
    let (a, b, c) = parse_colour(&settings.resting_colour);
    let yao_leds = controller.leds_mut(0);
    for num in 0..yao_leds.len() - 1 {
        yao_leds[num as usize] = [a, b, c, 0];
    }
    let li_leds = controller.leds_mut(1);
    for num in 0..li_leds.len() - 1 {
        li_leds[num as usize] = [a, b, c, 0];
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
