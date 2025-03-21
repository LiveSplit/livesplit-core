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
#[unsafe(no_mangle)]
pub extern "C" fn TimeSpan_clone(this: &TimeSpan) -> OwnedTimeSpan {
    Box::new(*this)
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn TimeSpan_drop(this: OwnedTimeSpan) {
    drop(this);
}

/// Creates a new Time Span from a given amount of seconds.
#[unsafe(no_mangle)]
pub extern "C" fn TimeSpan_from_seconds(seconds: f64) -> OwnedTimeSpan {
    Box::new(TimeSpan::from_seconds(seconds))
}

/// Parses a Time Span from a string. Returns <NULL> if the time can't be
/// parsed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TimeSpan_parse(text: *const c_char) -> NullableOwnedTimeSpan {
    // SAFETY: The caller guarantees that `text` is valid.
    unsafe { str(text).parse().ok().map(Box::new) }
}

/// Returns the total amount of seconds (including decimals) this Time Span
/// represents.
#[unsafe(no_mangle)]
pub extern "C" fn TimeSpan_total_seconds(this: &TimeSpan) -> f64 {
    this.total_seconds()
}

/// Returns the total amount of whole seconds (excluding decimals) this Time
/// Span represents.
#[unsafe(no_mangle)]
pub extern "C" fn TimeSpan_whole_seconds(this: &TimeSpan) -> i64 {
    this.to_seconds_and_subsec_nanoseconds().0
}

/// Returns the number of nanoseconds past the last full second that makes up
/// the Time Span.
#[unsafe(no_mangle)]
pub extern "C" fn TimeSpan_subsec_nanoseconds(this: &TimeSpan) -> i32 {
    this.to_seconds_and_subsec_nanoseconds().1
}

/// Changes a Time Span by adding a Time Span onto it.
#[unsafe(no_mangle)]
pub extern "C" fn TimeSpan_add_assign(this: &mut TimeSpan, other: &TimeSpan) {
    *this += *other;
}
