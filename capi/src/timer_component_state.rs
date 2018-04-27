//! The state object describes the information to visualize for this component.

use super::{acc, output_str, output_vec, own_drop};
use livesplit_core::component::timer::State as TimerComponentState;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedTimerComponentState = *mut TimerComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn TimerComponentState_drop(this: OwnedTimerComponentState) {
    own_drop(this);
}

/// The time shown by the component without the fractional part.
#[no_mangle]
pub unsafe extern "C" fn TimerComponentState_time(
    this: *const TimerComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}

/// The fractional part of the time shown (including the dot).
#[no_mangle]
pub unsafe extern "C" fn TimerComponentState_fraction(
    this: *const TimerComponentState,
) -> *const c_char {
    output_str(&acc(this).fraction)
}

/// The semantic coloring information the time carries.
#[no_mangle]
pub unsafe extern "C" fn TimerComponentState_semantic_color(
    this: *const TimerComponentState,
) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", acc(this).semantic_color).unwrap())
}
