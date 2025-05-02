use heapless::String;
use core::fmt::Write;

pub const TIMEOUT: u32 = 5000;

// Format message for sending
pub fn make_msg(start: u64, stop: u64, value: f64) -> String<128> {
    let mut msg = String::<128>::new();
    write!(
        &mut msg,
        r#"{{"start": {}, "stop": {}, "value": {}}}"#,
        start, stop - TIMEOUT as u64, value
    ).unwrap();
    msg
}
