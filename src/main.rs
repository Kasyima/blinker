#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output},
    prelude::*,
};
// use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();

    // Set GPIO4 as an output, and set its state high initially.
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led_pin = Output::new(io.pins.gpio1, Level::Low);

    loop {
        led_pin.set_high();

        delay.delay_millis(1000u32);

        led_pin.set_low();

        delay.delay_millis(1000u32);
    }
}
