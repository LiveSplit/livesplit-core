#[no_mangle]
pub extern "C" fn configure() {
    let mut buf = [0; 32];
    getrandom::getrandom(&mut buf).unwrap();
    assert_ne!(buf, [0; 32]);
}

fn main() {}
