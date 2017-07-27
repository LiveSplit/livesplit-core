use livesplit_core::component::title::State as TitleComponentState;
use super::{own_drop, acc, output_str};
use libc::c_char;
use std::ptr;

pub type OwnedTitleComponentState = *mut TitleComponentState;

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_drop(this: OwnedTitleComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_icon_change(
    this: *const TitleComponentState,
) -> *const c_char {
    acc(this)
        .icon_change
        .as_ref()
        .map_or_else(ptr::null, output_str)
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_line1(
    this: *const TitleComponentState,
) -> *const c_char {
    output_str(&acc(this).line1)
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_line2(
    this: *const TitleComponentState,
) -> *const c_char {
    acc(this).line2.as_ref().map_or_else(ptr::null, output_str)
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_is_centered(this: *const TitleComponentState) -> bool {
    acc(this).is_centered
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_shows_finished_runs(
    this: *const TitleComponentState,
) -> bool {
    acc(this).finished_runs.is_some()
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_finished_runs(
    this: *const TitleComponentState,
) -> u32 {
    acc(this).finished_runs.unwrap_or_default()
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_shows_attempts(
    this: *const TitleComponentState,
) -> bool {
    acc(this).attempts.is_some()
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_attempts(this: *const TitleComponentState) -> u32 {
    acc(this).attempts.unwrap_or_default()
}
