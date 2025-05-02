use esp_hal::rtc_cntl::Rtc;
use critical_section::Mutex;
use core::cell::RefCell;
use esp_hal::peripherals::LPWR;

static RTC_INSTANCE: Mutex<RefCell<Option<Rtc>>> = Mutex::new(RefCell::new(None));

static LAST_ISR_TIME: Mutex<RefCell<u64>> = Mutex::new(RefCell::new(0));

// Start timer
pub fn init_rtc(lpwr: LPWR) {
    critical_section::with(|cs| {
        let mut rtc = RTC_INSTANCE.borrow_ref_mut(cs);
        if rtc.is_none() {
            *rtc = Some(Rtc::new(lpwr));
        }
    });
}

// Return current timestamp
pub fn get_current_time() -> u64 {
    critical_section::with(|cs| {
        RTC_INSTANCE.borrow_ref_mut(cs).as_mut().map(|rtc| rtc.get_time_ms()).unwrap_or(0)
    })
}

// Set latest ISR timestamp
pub fn set_last_event_time() {
    critical_section::with(|cs| {
        *LAST_ISR_TIME.borrow_ref_mut(cs) = get_current_time();
    });
}

// Return latest ISR timestamp
pub fn get_last_event_time() -> u64 {
    critical_section::with(|cs| *LAST_ISR_TIME.borrow_ref(cs))
}

// Allow access to timer
pub fn with_rtc<F: FnOnce(&mut Rtc)>(f: F) {
    critical_section::with(|cs| {
        if let Some(ref mut rtc) = RTC_INSTANCE.borrow_ref_mut(cs).as_mut() {
            f(rtc);
        }
    });
}
