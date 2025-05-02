#![no_std]
#![no_main]

use esp_hal::{
    clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*, system::SystemControl,
};

use esp_backtrace as _;
use esp_hal::gpio::Io;
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

    // Start timer
    board::timer::init_rtc(lpwr);
    let start = board::timer::get_current_time();

    // Start measurement ISR
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    board::io::init_isr(io);
    println!("Started Measurements");

    // Collecting measurements
    let mut diff: i64 = 0;
    while diff < (util::message::TIMEOUT as i64) {
        let current = board::timer::get_current_time() as i64;
        let last = board::timer::get_last_event_time() as i64;
        diff = current - last;
    }
    board::io::stop_isr();
    println!("Stopped Measurements");

    // Form data package
    let stop = board::timer::get_current_time();
    let value = board::sensor::get_measurement();
    let msg = util::message::make_msg(start, stop, value);

    // Send data package
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
    power::sleep::configure_wakeup();
    println!("Good Night!");
    power::sleep::enter();
}
