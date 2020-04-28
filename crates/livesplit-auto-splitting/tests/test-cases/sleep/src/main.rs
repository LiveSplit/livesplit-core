use std::{thread, time};

#[no_mangle]
pub extern "C" fn configure() {
    thread::yield_now();
    thread::sleep(time::Duration::from_secs(10));
}

fn main() {}
