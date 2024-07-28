// Most of this is copied from the rppal main github site as I learn how to use the library for various things.
// See https://github.com/golemparts/rppal

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::{DeviceInfo, Model};

// Use BCM Numbering
const GPIO_LED: u8  = 23;

// Set constants
const FLASH_TIME_MS: u64 = 500;




fn main() -> Result<(), Box<dyn Error>> {

    // SEtup Pi Hooks
    let rpi_device: DeviceInfo = DeviceInfo::new().expect("No Device Found");
    let rpi_type: Model   = rpi_device.model();
    let rpi_gpio: Gpio = Gpio::new().expect("Cannot access GPIO");
    
    println!("Flasing LED at pin {} on {}.", GPIO_LED, rpi_type);

    let mut led_pin = rpi_gpio.get(GPIO_LED).expect("Cannot access pin").into_output();

    // Flash LED every so often
    
    led_pin.set_high();

    thread::sleep(Duration::from_millis(FLASH_TIME_MS));

    led_pin.set_low();

    Ok(())
}
