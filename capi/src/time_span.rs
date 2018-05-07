//! A Time Span represents a certain span of time.

use super::{acc, alloc, own_drop, str};
use livesplit_core::TimeSpan;
use std::os::raw::c_char;
use std::ptr;

/// type
pub type NullableTimeSpan = TimeSpan;
/// type
pub type OwnedTimeSpan = *mut TimeSpan;
/// type
pub type NullableOwnedTimeSpan = *mut TimeSpan;

/// Clones the Time Span.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_clone(this: *const TimeSpan) -> OwnedTimeSpan {
    alloc(*acc(this))
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_drop(this: OwnedTimeSpan) {
    own_drop(this);
}

/// Creates a new Time Span from a given amount of seconds.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_from_seconds(seconds: f64) -> OwnedTimeSpan {
    alloc(TimeSpan::from_seconds(seconds))
}

/// Parses a Time Span from a string. Returns <NULL> if the time can't be
/// parsed.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_parse(text: *const c_char) -> NullableOwnedTimeSpan {
    str(text).parse().ok().map_or_else(ptr::null_mut, alloc)
}

/// Returns the total amount of seconds (including decimals) this Time Span
/// represents.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_total_seconds(this: *const TimeSpan) -> f64 {
    acc(this).total_seconds()
}
