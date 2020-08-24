use crate::errors::IOracleResult;
// use crate::oracle::iching::Hexagram;

pub fn ask_wires() -> IOracleResult<String> {
    println!("wires");

    Ok("ok".to_string())
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
