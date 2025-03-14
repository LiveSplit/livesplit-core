use std::{
    thread,
    time::{Duration, Instant, SystemTime},
};

#[unsafe(no_mangle)]
pub extern "C" fn update() {
    assert!(SystemTime::now() > SystemTime::UNIX_EPOCH);

    let earlier = Instant::now();
    thread::sleep(Duration::from_secs(1));
    assert_ne!(earlier.elapsed(), Duration::from_secs(0));
}

fn main() {}
