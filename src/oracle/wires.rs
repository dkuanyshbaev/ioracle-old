use crate::oracle::iching::Line;
use rppal::gpio::Gpio;
// use std::error::Error;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;
use std::thread;
use std::time::Duration;

const LEDS_IN_LINE: i32 = 144;

pub fn yin(line_num: u8) {
    let controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(12)
                .count(3 * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build(),
        )
        .build();

    if let Ok(mut c) = controller {
        let leds = c.leds_mut(0);

        for line in 0..3 {
            for num in 0..LEDS_IN_LINE {
                leds[num as usize] = [255, 255, 255, 0];
            }
        }

        match c.render() {
            Ok(_) => println!("ok!"),
            Err(e) => println!("{:?}", e),
        };
    }

    // thread::sleep(Duration::from_secs(5));

    println!("yin");
    println!("light line number {}", line_num);
}

pub fn yang(line_num: u8) {
    println!("yang");
    println!("light line number {}", line_num);
}

pub fn on(pin: u8) {
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_high();
        }
    }
}

pub fn off(pin: u8) {
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_low();
        }
    }
}

pub fn heaven_on(_colour: String, pin: u8) {
    println!("----> heaven on, pin {}", pin);

    on(pin);
}

pub fn heaven_off(pin: u8) {
    println!("----> heaven off, pin {}", pin);

    off(pin);
}

pub fn cloud_on(_colour: String, pin: u8) {
    println!("----> cloud on, pin {}", pin);

    on(pin);
}

pub fn cloud_off(pin: u8) {
    println!("----> cloud off, pin {}", pin);

    off(pin);
}

pub fn sun_on(_colour: String, pin: u8) {
    println!("----> sun on, pin {}", pin);

    on(pin);
}

pub fn sun_off(pin: u8) {
    println!("----> sun off, pin {}", pin);

    off(pin);
}

pub fn wind_on(_colour: String, pin: u8) {
    println!("----> wind on, pin {}", pin);

    on(pin);
}

pub fn wind_off(pin: u8) {
    println!("----> wind off, pin {}", pin);

    off(pin);
}

pub fn thunder_on(_colour: String, sound: String) {
    println!("----> thunder on");

    println!("play {}", sound);
}

pub fn thunder_off(sound: String) {
    println!("----> thunder off");

    println!("stop play {}", sound);
}

pub fn water_on(_colour: String, pin: u8) {
    println!("----> water on, pin {}", pin);

    on(pin);
}

pub fn water_off(pin: u8) {
    println!("----> water off, pin {}", pin);

    off(pin);
}

pub fn mountain_on(_colour: String, sound: String) {
    println!("----> mountain on");

    println!("play {}", sound);
}

pub fn mountain_off(sound: String) {
    println!("----> mountain off");

    println!("stop play {}", sound);
}

pub fn earth_on(_colour: String, _pin: u8) {
    println!("----> earth on");

    // on_off(pin);
}

pub fn earth_off(_pin: u8) {
    println!("----> earth off");

    // on_off(pin);
}
