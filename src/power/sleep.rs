use esp_hal::rtc_cntl::sleep::GpioWakeupSource;
use esp_hal::reset;
use esp_hal::gpio::{WakeEvent};

use crate::board::timer::with_rtc;
use crate::board::io::with_gpio0;

// Enter sleep
pub fn enter() -> ! {
    with_rtc(|rtc| {
        let wakeup = GpioWakeupSource::new();
        rtc.sleep_light(&[&wakeup]);
        reset::software_reset_cpu();
    });

    panic!("Sleep")
}

// Create wake up condition
pub fn configure_wakeup() {
    with_gpio0(|pin| {
        let event = if pin.is_high() {
            WakeEvent::LowLevel
        } else {
            WakeEvent::HighLevel
        };
        pin.wakeup_enable(true, event);
    });
}
