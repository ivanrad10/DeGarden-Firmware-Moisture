use core::fmt::Write;
use heapless::String;

// Format message for sending
pub fn make_msg(value: f64) -> String<128> {
    let mut msg = String::<128>::new();
    write!(&mut msg, r#"{{"value": {}}}"#, value).unwrap();
    msg
}
