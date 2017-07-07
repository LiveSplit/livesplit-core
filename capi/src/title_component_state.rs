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
        .map_or_else(ptr::null, |s| output_str(s))
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_game(
    this: *const TitleComponentState,
) -> *const c_char {
    output_str(&acc(this).game)
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_category(
    this: *const TitleComponentState,
) -> *const c_char {
    output_str(&acc(this).category)
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
