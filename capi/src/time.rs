//! A time that can store a Real Time and a Game Time. Both of them are
//! optional.

use crate::time_span::NullableTimeSpan;
use livesplit_core::{Time, TimingMethod};
use std::ptr;

/// type
pub type OwnedTime = Box<Time>;

/// Clones the time.
#[no_mangle]
pub extern "C" fn Time_clone(this: &Time) -> OwnedTime {
    Box::new(*this)
}

/// drop
#[no_mangle]
pub extern "C" fn Time_drop(this: OwnedTime) {
    drop(this);
}

/// The Real Time value. This may be <NULL> if this time has no Real Time value.
#[no_mangle]
pub extern "C" fn Time_real_time(this: &Time) -> *const NullableTimeSpan {
    this.real_time
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}

/// The Game Time value. This may be <NULL> if this time has no Game Time value.
#[no_mangle]
pub extern "C" fn Time_game_time(this: &Time) -> *const NullableTimeSpan {
    this.game_time
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}

/// Access the time's value for the timing method specified.
#[no_mangle]
pub extern "C" fn Time_index(this: &Time, timing_method: TimingMethod) -> *const NullableTimeSpan {
    this[timing_method]
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}
