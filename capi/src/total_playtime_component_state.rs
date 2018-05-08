//! The state object describes the information to visualize for this component.

use super::output_str;
use livesplit_core::component::total_playtime::State as TotalPlaytimeComponentState;
use std::os::raw::c_char;

/// type
pub type OwnedTotalPlaytimeComponentState = Box<TotalPlaytimeComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn TotalPlaytimeComponentState_drop(this: OwnedTotalPlaytimeComponentState) {
    drop(this);
}

/// The label's text.
#[no_mangle]
pub extern "C" fn TotalPlaytimeComponentState_text(
    this: &TotalPlaytimeComponentState,
) -> *const c_char {
    output_str(&this.text)
}

/// The total playtime.
#[no_mangle]
pub extern "C" fn TotalPlaytimeComponentState_time(
    this: &TotalPlaytimeComponentState,
) -> *const c_char {
    output_str(&this.time)
}
