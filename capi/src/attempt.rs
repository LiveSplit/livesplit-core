use livesplit_core::{Attempt, Time};
use super::{acc, output_time};

pub type OwnedAttempt = *mut Attempt;

#[no_mangle]
pub unsafe extern "C" fn Attempt_index(this: *const Attempt) -> i32 {
    acc(this).index()
}

#[no_mangle]
pub unsafe extern "C" fn Attempt_time(this: *const Attempt) -> *const Time {
    output_time(acc(this).time())
}
