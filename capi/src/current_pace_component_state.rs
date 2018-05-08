//! The state object describes the information to visualize for this component.

use super::output_str;
use livesplit_core::component::current_pace::State as CurrentPaceComponentState;
use std::os::raw::c_char;

/// type
pub type OwnedCurrentPaceComponentState = Box<CurrentPaceComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn CurrentPaceComponentState_drop(this: OwnedCurrentPaceComponentState) {
    drop(this);
}

/// The label's text.
#[no_mangle]
pub extern "C" fn CurrentPaceComponentState_text(
    this: &CurrentPaceComponentState,
) -> *const c_char {
    output_str(&this.text)
}

/// The current pace.
#[no_mangle]
pub extern "C" fn CurrentPaceComponentState_time(
    this: &CurrentPaceComponentState,
) -> *const c_char {
    output_str(&this.time)
}
