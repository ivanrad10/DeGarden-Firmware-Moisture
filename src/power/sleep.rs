use core::time::Duration;

use esp_hal::rtc_cntl::sleep::TimerWakeupSource;

use crate::board;

// Enter sleep
pub fn enter() -> ! {
    const SLEEP_DURATION: &str = env!("SLEEP_DURATION");
    let sleep_duration: u64 = SLEEP_DURATION.parse::<u64>().unwrap();

    let wakeup_source = TimerWakeupSource::new(Duration::new(sleep_duration, 0));
    board::timer::with_rtc(|rtc| {
        rtc.sleep_deep(&[&wakeup_source]);
    });
    panic!("Sleeping");
}
