#![no_std]
#![no_main]

use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::Io, peripherals::Peripherals, prelude::*,
    system::SystemControl,
};

use esp_backtrace as _;
use esp_println::println;

mod board;
mod comms;
mod power;
mod util;

#[entry]
fn main() -> ! {
    // Initialize
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    println!("Waking Up!");

    // Take peripherals
    let mut timg0 = peripherals.TIMG0;
    let mut rng = peripherals.RNG;
    let radio_clk = peripherals.RADIO_CLK;
    let mut wifi = peripherals.WIFI;
    let lpwr = peripherals.LPWR;
    let adc1 = peripherals.ADC1;

    // Take measurement
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    println!("Measurement Taken");

    // Form data package
    let value = board::sensor::get_measurement(io, adc1);
    let msg = util::message::make_msg(value);

    // Send data package
    board::timer::init_rtc(lpwr);

    comms::send(
        &mut timg0,
        &mut rng,
        radio_clk,
        &mut wifi,
        &clocks,
        &delay,
        msg.as_str(),
    );

    // Go to sleep
    println!("Good Night!");
    power::sleep::enter();
}
