use core::time::Duration;

use esp_hal::peripherals::LPWR;
use esp_hal::rtc_cntl::sleep::TimerWakeupSource;
use esp_hal::rtc_cntl::Rtc;

const INTERVAL: u64 = 5;

// Enter sleep
pub fn enter(rtc_cntl: LPWR) -> ! {
    let wakeup_source = TimerWakeupSource::new(Duration::new(INTERVAL, 0));
    let mut rtc = Rtc::new(rtc_cntl);
    rtc.sleep_deep(&[&wakeup_source]);
}
