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

/// The semantic coloring information the time carries.
#[unsafe(no_mangle)]
pub extern "C" fn TimerComponentState_semantic_color(this: &TimerComponentState) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", this.semantic_color).unwrap())
}
