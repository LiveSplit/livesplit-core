use livesplit_core::{Attempt, Time, TimeSpan};
use super::{acc, output_time, output_time_span, alloc};
use std::ptr;
use atomic_date_time::OwnedAtomicDateTime;

pub type OwnedAttempt = *mut Attempt;

#[no_mangle]
pub unsafe extern "C" fn Attempt_index(this: *const Attempt) -> i32 {
    acc(this).index()
}

#[no_mangle]
pub unsafe extern "C" fn Attempt_time(this: *const Attempt) -> *const Time {
    output_time(acc(this).time())
}

#[no_mangle]
pub unsafe extern "C" fn Attempt_pause_time(this: *const Attempt) -> *const TimeSpan {
    if let Some(pause_time) = acc(this).pause_time() {
        output_time_span(pause_time)
    } else {
        ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn Attempt_started(this: *const Attempt) -> OwnedAtomicDateTime {
    if let Some(date_time) = acc(this).started() {
        alloc(date_time)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn Attempt_ended(this: *const Attempt) -> OwnedAtomicDateTime {
    if let Some(date_time) = acc(this).ended() {
        alloc(date_time)
    } else {
        ptr::null_mut()
    }
}
