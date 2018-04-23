//! A time that can store a Real Time and a Game Time. Both of them are
//! optional.

use super::{acc, alloc, own_drop};
use livesplit_core::{Time, TimingMethod};
use std::ptr;
use time_span::NullableTimeSpan;

/// type
pub type OwnedTime = *mut Time;

/// Clones the time.
#[no_mangle]
pub unsafe extern "C" fn Time_clone(this: *const Time) -> OwnedTime {
    alloc(*acc(this))
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn Time_drop(this: OwnedTime) {
    own_drop(this);
}

/// The Real Time value. This may be <NULL> if this time has no Real Time value.
#[no_mangle]
pub unsafe extern "C" fn Time_real_time(this: *const Time) -> *const NullableTimeSpan {
    acc(this)
        .real_time
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}

/// The Game Time value. This may be <NULL> if this time has no Game Time value.
#[no_mangle]
pub unsafe extern "C" fn Time_game_time(this: *const Time) -> *const NullableTimeSpan {
    acc(this)
        .game_time
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}

/// Access the time's value for the timing method specified.
#[no_mangle]
pub unsafe extern "C" fn Time_index(
    this: *const Time,
    timing_method: TimingMethod,
) -> *const NullableTimeSpan {
    acc(this)[timing_method]
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}
