//! The state object describes the information to visualize for this component.

use super::{acc, output_str, own_drop};
use livesplit_core::component::current_pace::State as CurrentPaceComponentState;
use std::os::raw::c_char;

/// type
pub type OwnedCurrentPaceComponentState = *mut CurrentPaceComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponentState_drop(this: OwnedCurrentPaceComponentState) {
    own_drop(this);
}

/// The label's text.
#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponentState_text(
    this: *const CurrentPaceComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

/// The current pace.
#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponentState_time(
    this: *const CurrentPaceComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}
