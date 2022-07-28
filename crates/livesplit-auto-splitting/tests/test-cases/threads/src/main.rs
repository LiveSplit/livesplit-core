#[no_mangle]
pub extern "C" fn update() {
    assert!(std::thread::spawn(|| {}).join().is_err());
}

fn main() {}
