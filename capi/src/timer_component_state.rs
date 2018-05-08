//! The state object describes the information to visualize for this component.

use super::{output_str, output_vec};
use livesplit_core::component::timer::State as TimerComponentState;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedTimerComponentState = Box<TimerComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn TimerComponentState_drop(this: OwnedTimerComponentState) {
    drop(this);
}

/// The time shown by the component without the fractional part.
#[no_mangle]
pub extern "C" fn TimerComponentState_time(this: &TimerComponentState) -> *const c_char {
    output_str(&this.time)
}

/// The fractional part of the time shown (including the dot).
#[no_mangle]
pub extern "C" fn TimerComponentState_fraction(this: &TimerComponentState) -> *const c_char {
    output_str(&this.fraction)
}

/// The semantic coloring information the time carries.
#[no_mangle]
pub extern "C" fn TimerComponentState_semantic_color(this: &TimerComponentState) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", this.semantic_color).unwrap())
}
