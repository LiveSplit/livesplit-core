#[unsafe(no_mangle)]
pub extern "C" fn update() {
    loop {
        std::thread::yield_now();
    }
}

fn main() {}
