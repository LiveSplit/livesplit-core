//! The state object describes the information to visualize for this component.

use livesplit_core::component::title::State as TitleComponentState;
use super::{acc, output_str, own_drop, Nullablec_char};
use std::os::raw::c_char;
use std::ptr;

/// type
pub type OwnedTitleComponentState = *mut TitleComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_drop(this: OwnedTitleComponentState) {
    own_drop(this);
}

/// The game's icon encoded as a Data URL. This value is only specified whenever
/// the icon changes. If you explicitly want to query this value, remount the
/// component. The String itself may be empty. This indicates that there is no
/// icon. If no change occurred, <NULL> is returned instead.
#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_icon_change(
    this: *const TitleComponentState,
) -> *const Nullablec_char {
    acc(this)
        .icon_change
        .as_ref()
        .map_or_else(ptr::null, output_str)
}

/// The first title line to show. This is either the game's name, or a
/// combination of the game's name and the category.
#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_line1(
    this: *const TitleComponentState,
) -> *const c_char {
    output_str(&acc(this).line1)
}

/// By default the category name is shown on the second line. Based on the
/// settings, it can however instead be shown in a single line together with
/// the game name. In that case <NULL> is returned instead.
#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_line2(
    this: *const TitleComponentState,
) -> *const Nullablec_char {
    acc(this).line2.as_ref().map_or_else(ptr::null, output_str)
}

/// Specifies whether the title should centered or aligned to the left
/// instead.
#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_is_centered(this: *const TitleComponentState) -> bool {
    acc(this).is_centered
}

/// Returns whether the amount of successfully finished attempts is supposed to
/// be shown.
#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_shows_finished_runs(
    this: *const TitleComponentState,
) -> bool {
    acc(this).finished_runs.is_some()
}

/// Returns the amount of successfully finished attempts.
#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_finished_runs(
    this: *const TitleComponentState,
) -> u32 {
    acc(this).finished_runs.unwrap_or_default()
}

/// Returns whether the amount of total attempts is supposed to be shown.
#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_shows_attempts(
    this: *const TitleComponentState,
) -> bool {
    acc(this).attempts.is_some()
}

/// Returns the amount of total attempts.
#[no_mangle]
pub unsafe extern "C" fn TitleComponentState_attempts(this: *const TitleComponentState) -> u32 {
    acc(this).attempts.unwrap_or_default()
}
