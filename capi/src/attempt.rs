//! An Attempt describes information about an attempt to run a specific category
//! by a specific runner in the past. Every time a new attempt is started and
//! then reset, an Attempt describing general information about it is created.

use super::{output_time, output_time_span};
use crate::atomic_date_time::NullableOwnedAtomicDateTime;
use crate::time_span::NullableTimeSpan;
use livesplit_core::{Attempt, Time};
use std::ptr;

/// type
pub type OwnedAttempt = Box<Attempt>;

/// Accesses the unique index of the attempt. This index is unique for the
/// Run, not for all of them.
#[no_mangle]
pub extern "C" fn Attempt_index(this: &Attempt) -> i32 {
    this.index()
}

/// Accesses the split time of the last segment. If the attempt got reset
/// early and didn't finish, this may be empty.
#[no_mangle]
pub extern "C" fn Attempt_time(this: &Attempt) -> *const Time {
    output_time(this.time())
}

/// Accesses the amount of time the attempt has been paused for. If it is not
/// known, this returns <NULL>. This means that it may not necessarily be
/// possible to differentiate whether a Run has not been paused or it simply
/// wasn't stored.
#[no_mangle]
pub extern "C" fn Attempt_pause_time(this: &Attempt) -> *const NullableTimeSpan {
    if let Some(pause_time) = this.pause_time() {
        output_time_span(pause_time)
    } else {
        ptr::null()
    }
}

/// Accesses the point in time the attempt was started at. This returns <NULL>
/// if this information is not known.
#[no_mangle]
pub extern "C" fn Attempt_started(this: &Attempt) -> NullableOwnedAtomicDateTime {
    if let Some(date_time) = this.started() {
        Some(Box::new(date_time))
    } else {
        None
    }
}

/// Accesses the point in time the attempt was ended at. This returns <NULL> if
/// this information is not known.
#[no_mangle]
pub extern "C" fn Attempt_ended(this: &Attempt) -> NullableOwnedAtomicDateTime {
    if let Some(date_time) = this.ended() {
        Some(Box::new(date_time))
    } else {
        None
    }
}
