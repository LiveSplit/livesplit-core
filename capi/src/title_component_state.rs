//! The state object describes the information to visualize for this component.

use super::{output_str, Nullablec_char};
use livesplit_core::component::title::State as TitleComponentState;
use std::{os::raw::c_char, ptr};

/// type
pub type OwnedTitleComponentState = Box<TitleComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn TitleComponentState_drop(this: OwnedTitleComponentState) {
    drop(this);
}

/// The game icon to show. The associated image can be looked up in the image
/// cache. The image may be the empty image. This indicates that there is no
/// icon.
#[no_mangle]
pub extern "C" fn TitleComponentState_icon(this: &TitleComponentState) -> *const c_char {
    output_str(this.icon.format_str(&mut [0; 64]))
}

/// The first title line to show. This is either the game's name, or a
/// combination of the game's name and the category.
#[no_mangle]
pub extern "C" fn TitleComponentState_line1(this: &TitleComponentState) -> *const c_char {
    // FIXME: Add API for querying the abbreviations.
    output_str(this.line1.last().unwrap())
}

/// By default the category name is shown on the second line. Based on the
/// settings, it can however instead be shown in a single line together with
/// the game name. In that case <NULL> is returned instead.
#[no_mangle]
pub extern "C" fn TitleComponentState_line2(this: &TitleComponentState) -> *const Nullablec_char {
    // FIXME: Add API for querying the abbreviations.
    this.line2.last().map_or_else(ptr::null, output_str)
}

/// Specifies whether the title should centered or aligned to the left
/// instead.
#[no_mangle]
pub extern "C" fn TitleComponentState_is_centered(this: &TitleComponentState) -> bool {
    this.is_centered
}

/// Returns whether the amount of successfully finished attempts is supposed to
/// be shown.
#[no_mangle]
pub extern "C" fn TitleComponentState_shows_finished_runs(this: &TitleComponentState) -> bool {
    this.finished_runs.is_some()
}

/// Returns the amount of successfully finished attempts.
#[no_mangle]
pub extern "C" fn TitleComponentState_finished_runs(this: &TitleComponentState) -> u32 {
    this.finished_runs.unwrap_or_default()
}

/// Returns whether the amount of total attempts is supposed to be shown.
#[no_mangle]
pub extern "C" fn TitleComponentState_shows_attempts(this: &TitleComponentState) -> bool {
    this.attempts.is_some()
}

/// Returns the amount of total attempts.
#[no_mangle]
pub extern "C" fn TitleComponentState_attempts(this: &TitleComponentState) -> u32 {
    this.attempts.unwrap_or_default()
}
