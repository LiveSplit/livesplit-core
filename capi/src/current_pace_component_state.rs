use livesplit_core::component::current_pace::State as CurrentPaceComponentState;
use super::{own_drop, acc, output_str};
use libc::c_char;

pub type OwnedCurrentPaceComponentState = *mut CurrentPaceComponentState;

#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponentState_drop(this: OwnedCurrentPaceComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponentState_text(
    this: *const CurrentPaceComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponentState_time(
    this: *const CurrentPaceComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}
