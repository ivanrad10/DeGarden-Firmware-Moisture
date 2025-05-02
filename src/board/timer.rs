use core::cell::RefCell;
use critical_section::Mutex;
use esp_hal::peripherals::LPWR;
use esp_hal::rtc_cntl::Rtc;

static RTC_INSTANCE: Mutex<RefCell<Option<Rtc>>> = Mutex::new(RefCell::new(None));

// Start timer
pub fn init_rtc(lpwr: LPWR) {
    critical_section::with(|cs| {
        let mut rtc = RTC_INSTANCE.borrow_ref_mut(cs);
        if rtc.is_none() {
            *rtc = Some(Rtc::new(lpwr));
        }
    });
}

// // Return current timestamp
// pub fn get_current_time() -> u64 {
//     critical_section::with(|cs| {
//         RTC_INSTANCE
//             .borrow_ref_mut(cs)
//             .as_mut()
//             .map(|rtc| rtc.get_time_ms())
//             .unwrap_or(0)
//     })
// }

// Allow access to timer
pub fn with_rtc<F: FnOnce(&mut Rtc)>(f: F) {
    critical_section::with(|cs| {
        if let Some(ref mut rtc) = RTC_INSTANCE.borrow_ref_mut(cs).as_mut() {
            f(rtc);
        }
    });
}
