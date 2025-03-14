use std::{thread, time};

#[unsafe(no_mangle)]
pub extern "C" fn update() {
    thread::yield_now();
    thread::sleep(time::Duration::from_secs(10));
}

fn main() {}
