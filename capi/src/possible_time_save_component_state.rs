//! The state object describes the information to visualize for this component.

use super::output_str;
use livesplit_core::component::possible_time_save::State as PossibleTimeSaveComponentState;
use std::os::raw::c_char;

/// type
pub type OwnedPossibleTimeSaveComponentState = Box<PossibleTimeSaveComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn PossibleTimeSaveComponentState_drop(this: OwnedPossibleTimeSaveComponentState) {
    drop(this);
}

/// The label's text.
#[no_mangle]
pub extern "C" fn PossibleTimeSaveComponentState_text(
    this: &PossibleTimeSaveComponentState,
) -> *const c_char {
    output_str(&this.text)
}

/// The current possible time save.
#[no_mangle]
pub extern "C" fn PossibleTimeSaveComponentState_time(
    this: &PossibleTimeSaveComponentState,
) -> *const c_char {
    output_str(&this.time)
}
