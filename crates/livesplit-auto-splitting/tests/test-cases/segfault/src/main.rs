#[unsafe(no_mangle)]
pub unsafe extern "C" fn update() {
    let some_large_ptr = (usize::MAX / 2) as *mut u64;
    *some_large_ptr = !0;
}

fn main() {}
