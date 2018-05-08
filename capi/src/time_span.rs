//! A Time Span represents a certain span of time.

use super::str;
use livesplit_core::TimeSpan;
use std::os::raw::c_char;

/// type
pub type NullableTimeSpan = TimeSpan;
/// type
pub type OwnedTimeSpan = Box<TimeSpan>;
/// type
pub type NullableOwnedTimeSpan = Option<OwnedTimeSpan>;

/// Clones the Time Span.
#[no_mangle]
pub extern "C" fn TimeSpan_clone(this: &TimeSpan) -> OwnedTimeSpan {
    Box::new(*this)
}

/// drop
#[no_mangle]
pub extern "C" fn TimeSpan_drop(this: OwnedTimeSpan) {
    drop(this);
}

/// Creates a new Time Span from a given amount of seconds.
#[no_mangle]
pub extern "C" fn TimeSpan_from_seconds(seconds: f64) -> OwnedTimeSpan {
    Box::new(TimeSpan::from_seconds(seconds))
}

/// Parses a Time Span from a string. Returns <NULL> if the time can't be
/// parsed.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_parse(text: *const c_char) -> NullableOwnedTimeSpan {
    str(text).parse().ok().map(Box::new)
}

/// Returns the total amount of seconds (including decimals) this Time Span
/// represents.
#[no_mangle]
pub extern "C" fn TimeSpan_total_seconds(this: &TimeSpan) -> f64 {
    this.total_seconds()
}
