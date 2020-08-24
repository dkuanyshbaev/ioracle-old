pub fn heaven(_colour: String, pin: u8) {
    println!("heaven............");
    println!("on {}", pin);
}

pub fn cloud(_colour: String, pin: u8) {
    println!("cloud............");
    println!("on {}", pin);
}

pub fn sun(_colour: String, pin: u8) {
    println!("sun............");
    println!("on {}", pin);
}

pub fn wind(_colour: String, pin: u8) {
    println!("ufffffffffffff............");
    println!("on {}", pin);
}

pub fn thunder(_colour: String, sound: String) {
    println!("thunder............");
    println!("play {}", sound);
}

pub fn water(_colour: String, pin: u8) {
    println!("water............");
    println!("on {}", pin);
}

pub fn mountain(_colour: String, sound: String) {
    println!("mountain............");
    println!("play {}", sound);
}

pub fn earth(_colour: String, pin: u8) {
    println!("earth............");
    println!("on {}", pin);
}

// use std::error::Error;
// use std::thread;
// use std::time::Duration;
//
// use rppal::gpio::Gpio;
// use rppal::system::DeviceInfo;
//
// // Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// const GPIO_LED: u8 = 23;
//
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
