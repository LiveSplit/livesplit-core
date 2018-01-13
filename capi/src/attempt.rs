//! An Attempt describes information about an attempt to run a specific category
//! by a specific runner in the past. Every time a new attempt is started and
//! then reset, an Attempt describing general information about it is created.

use livesplit_core::{Attempt, Time};
use super::{acc, alloc, output_time, output_time_span};
use std::ptr;
use atomic_date_time::NullableOwnedAtomicDateTime;
use time_span::NullableTimeSpan;

/// type
pub type OwnedAttempt = *mut Attempt;

/// Accesses the unique index of the attempt. This index is unique for the
/// Run, not for all of them.
#[no_mangle]
pub unsafe extern "C" fn Attempt_index(this: *const Attempt) -> i32 {
    acc(this).index()
}

/// Accesses the split time of the last segment. If the attempt got reset
/// early and didn't finish, this may be empty.
#[no_mangle]
pub unsafe extern "C" fn Attempt_time(this: *const Attempt) -> *const Time {
    output_time(acc(this).time())
}

/// Accesses the amount of time the attempt has been paused for. If it is not
/// known, this returns <NULL>. This means that it may not necessarily be
/// possible to differentiate whether a Run has not been paused or it simply
/// wasn't stored.
#[no_mangle]
pub unsafe extern "C" fn Attempt_pause_time(this: *const Attempt) -> *const NullableTimeSpan {
    if let Some(pause_time) = acc(this).pause_time() {
        output_time_span(pause_time)
    } else {
        ptr::null()
    }
}

/// Accesses the point in time the attempt was started at. This returns <NULL>
/// if this information is not known.
#[no_mangle]
pub unsafe extern "C" fn Attempt_started(this: *const Attempt) -> NullableOwnedAtomicDateTime {
    if let Some(date_time) = acc(this).started() {
        alloc(date_time)
    } else {
        ptr::null_mut()
    }
}

/// Accesses the point in time the attempt was ended at. This returns <NULL> if
/// this information is not known.
#[no_mangle]
pub unsafe extern "C" fn Attempt_ended(this: *const Attempt) -> NullableOwnedAtomicDateTime {
    if let Some(date_time) = acc(this).ended() {
        alloc(date_time)
    } else {
        ptr::null_mut()
    }
}
