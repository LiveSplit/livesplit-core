#[no_mangle]
pub extern "C" fn configure() {
    assert!(std::fs::write(
        "shouldnt_exist.txt",
        "This file should never exist. File a bug if you see this.",
    )
    .is_err());
}

fn main() {}
