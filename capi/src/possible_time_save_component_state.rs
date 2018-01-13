//! The state object describes the information to visualize for this component.

use livesplit_core::component::possible_time_save::State as PossibleTimeSaveComponentState;
use super::{acc, output_str, own_drop};
use libc::c_char;

/// type
pub type OwnedPossibleTimeSaveComponentState = *mut PossibleTimeSaveComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponentState_drop(
    this: OwnedPossibleTimeSaveComponentState,
) {
    own_drop(this);
}

/// The label's text.
#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponentState_text(
    this: *const PossibleTimeSaveComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

/// The current possible time save.
#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponentState_time(
    this: *const PossibleTimeSaveComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}
