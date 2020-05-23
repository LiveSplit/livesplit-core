#[no_mangle]
pub extern "C" fn configure() {
    loop {
        std::thread::yield_now();
    }
}

fn main() {}
