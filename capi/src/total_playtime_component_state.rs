use livesplit_core::component::total_playtime::State as TotalPlaytimeComponentState;
use super::{acc, output_str, own_drop};
use libc::c_char;

pub type OwnedTotalPlaytimeComponentState = *mut TotalPlaytimeComponentState;

#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponentState_drop(this: OwnedTotalPlaytimeComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponentState_text(
    this: *const TotalPlaytimeComponentState,
) -> *const c_char {
    output_str(&acc(&this).text)
}

#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponentState_time(
    this: *const TotalPlaytimeComponentState,
) -> *const c_char {
    output_str(&acc(&this).time)
}
