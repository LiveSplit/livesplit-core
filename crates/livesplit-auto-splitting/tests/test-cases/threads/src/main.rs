#[no_mangle]
pub extern "C" fn configure() {
    assert!(std::thread::spawn(|| {}).join().is_err());
}

fn main() {}
