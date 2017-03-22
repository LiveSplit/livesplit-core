use livesplit_core::component::title::State as TitleComponentState;
use super::{drop, acc, output_str};
use libc::c_char;
use std::ptr;

pub type OwnedTitleComponentState = *mut TitleComponentState;

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_drop(this: OwnedTitleComponentState) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_icon_change(this: *const TitleComponentState)
                                                         -> *const c_char {
    acc(this).icon_change.as_ref().map_or_else(ptr::null, |s| output_str(s))
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_game(this: *const TitleComponentState)
                                                  -> *const c_char {
    output_str(&acc(this).game)
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_category(this: *const TitleComponentState)
                                                      -> *const c_char {
    output_str(&acc(this).category)
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_attempts(this: *const TitleComponentState) -> u32 {
    acc(this).attempts
}
