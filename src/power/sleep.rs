use core::time::Duration;

use esp_hal::rtc_cntl::sleep::TimerWakeupSource;

use crate::board;

const INTERVAL: u64 = 5;

// Enter sleep
pub fn enter() -> ! {
    let wakeup_source = TimerWakeupSource::new(Duration::new(INTERVAL, 0));
    board::timer::with_rtc(|rtc| {
        rtc.sleep_deep(&[&wakeup_source]);
    });
    panic!("Sleeping");
}
