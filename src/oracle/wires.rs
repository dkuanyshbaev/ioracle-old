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
            0, // 6 channels?
            ChannelBuilder::new()
                .pin(12)
                .count(3 * LEDS_IN_LINE) // numbers of leds connected to channel
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build(),
        )
        .build();

    if let Ok(mut c) = controller {
        let leds = c.leds_mut(0); // channel?

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

pub fn on_off(pin: u8) {
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_high();
            thread::sleep(Duration::from_secs(5));
            pin.set_low();
        }
    }
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

pub fn heaven(_colour: String, pin: u8) {
    println!("----> heaven");

    on_off(pin);
}

pub fn cloud(_colour: String, pin: u8) {
    println!("----> cloud");

    on_off(pin);
}

pub fn sun(_colour: String, pin: u8) {
    println!("----> sun");

    on_off(pin);
}

pub fn wind(_colour: String, pin: u8) {
    println!("----> wind");

    on_off(pin);
}

pub fn thunder(_colour: String, sound: String) {
    println!("----> thunder");

    println!("play {}", sound);
}

pub fn water(_colour: String, pin: u8) {
    println!("----> water");

    on_off(pin);
}

pub fn mountain(_colour: String, sound: String) {
    println!("----> mountain");

    println!("play {}", sound);
}

pub fn earth(_colour: String, _pin: u8) {
    println!("----> earth");

    // on_off(pin);
}

// fn main() -> Result<(), Box<dyn Error>> {
//     println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
//     let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
//
//     for _ in 1..17 {
//         println!("on");
//         pin.set_high();
//         thread::sleep(Duration::from_secs(1));
//         println!("off");
//         pin.set_low();
//         thread::sleep(Duration::from_secs(1));
//     }
//
//     Ok(())
// }
