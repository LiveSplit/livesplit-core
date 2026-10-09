//! The state object describes the information to visualize for this component.

use super::{output_str, output_vec};
use livesplit_core::component::timer::State as TimerComponentState;
use std::{io::Write, os::raw::c_char};

/// type
pub type OwnedTimerComponentState = Box<TimerComponentState>;

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn TimerComponentState_drop(this: OwnedTimerComponentState) {
    drop(this);
}

/// The time shown by the component without the fractional part.
#[unsafe(no_mangle)]
pub extern "C" fn TimerComponentState_time(this: &TimerComponentState) -> *const c_char {
    output_str(&this.time)
}

/// The fractional part of the time shown (including the dot).
#[unsafe(no_mangle)]
pub extern "C" fn TimerComponentState_fraction(this: &TimerComponentState) -> *const c_char {
    output_str(&this.fraction)
}

/// The text to measure when sizing the time without its fractional part. Covers
/// both the expected maximum and the currently displayed time, with all digits
/// normalized to `8`.
#[unsafe(no_mangle)]
pub extern "C" fn TimerComponentState_time_size_hint(this: &TimerComponentState) -> *const c_char {
    output_str(&this.time_size_hint)
}

/// The fractional part (including the decimal separator) to reserve when
/// sizing the timer, with digits normalized to `8`. Measure it at the fraction's
/// scale.
#[unsafe(no_mangle)]
pub extern "C" fn TimerComponentState_fraction_size_hint(
    this: &TimerComponentState,
) -> *const c_char {
    output_str(&this.fraction_size_hint)
}

/// The semantic coloring information the time carries.
#[unsafe(no_mangle)]
pub extern "C" fn TimerComponentState_semantic_color(this: &TimerComponentState) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", this.semantic_color).unwrap())
}
