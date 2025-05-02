use critical_section::Mutex;
use core::cell::RefCell;

static ISR_COUNT: Mutex<RefCell<u64>> = Mutex::new(RefCell::new(0));
const FLOW_RESOLUTION: f64 = 2.25;

// Increment sensor register
pub fn increment() {
    critical_section::with(|cs| {
        *ISR_COUNT.borrow_ref_mut(cs) += 1;
    });
}

// Return current sensor reading
pub fn get_measurement() -> f64 {
    critical_section::with(|cs| *ISR_COUNT.borrow_ref(cs) as f64) * FLOW_RESOLUTION / 1000.0
}
