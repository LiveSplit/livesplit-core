//! A Time Span represents a certain span of time.

use livesplit_core::TimeSpan;
use super::{acc, alloc, own_drop};

/// type
pub type NullableTimeSpan = TimeSpan;
/// type
pub type OwnedTimeSpan = *mut TimeSpan;

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

/// Returns the total amount of seconds (including decimals) this Time Span
/// represents.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_total_seconds(this: *const TimeSpan) -> f64 {
    acc(this).total_seconds()
}
