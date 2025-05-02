use core::cell::RefCell;
use critical_section::Mutex;
use esp_hal::gpio::{Event, Gpio0, Input, Io, Pull};
use esp_hal::prelude::*;

static GPIO0: Mutex<RefCell<Option<Input<Gpio0>>>> = Mutex::new(RefCell::new(None));

// Start ISR
pub fn init_isr(mut io: Io) {
    io.set_interrupt_handler(handler);

    let mut pin = Input::new(io.pins.gpio0, Pull::Up);
    critical_section::with(|cs| {
        pin.listen(Event::AnyEdge);
        GPIO0.borrow_ref_mut(cs).replace(pin);
    });
}

// Stop ISR
pub fn stop_isr() {
    critical_section::with(|cs| {
        if let Some(ref mut pin) = GPIO0.borrow_ref_mut(cs).as_mut() {
            pin.unlisten();
        }
    });
}

/*
* ISR does following:
* - Captures ISR timestamp for idle control
* - Runs ISR counter for sensor readings
*/
#[handler]
fn handler() {
    crate::board::timer::set_last_event_time();
    crate::board::sensor::increment();

    critical_section::with(|cs| {
        GPIO0.borrow_ref_mut(cs).as_mut().unwrap().clear_interrupt();
    });
}

// Allow access to gpio0
pub fn with_gpio0<F: FnOnce(&mut Input<Gpio0>)>(f: F) {
    critical_section::with(|cs| {
        if let Some(ref mut pin) = GPIO0.borrow_ref_mut(cs).as_mut() {
            f(pin);
        }
    });
}
