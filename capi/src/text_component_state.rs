//! The state object describes the information to visualize for this component.

use super::output_str;
use livesplit_core::component::text::{State as TextComponentState, TextState};
use std::os::raw::c_char;

/// type
pub type OwnedTextComponentState = Box<TextComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn TextComponentState_drop(this: OwnedTextComponentState) {
    drop(this);
}

/// Accesses the left part of the text. If the text isn't split up, an empty
/// string is returned instead.
#[no_mangle]
pub extern "C" fn TextComponentState_left(this: &TextComponentState) -> *const c_char {
    if let TextState::Split(left, _) = &this.text {
        output_str(left)
    } else {
        output_str("")
    }
}

/// Accesses the right part of the text. If the text isn't split up, an empty
/// string is returned instead.
#[no_mangle]
pub extern "C" fn TextComponentState_right(this: &TextComponentState) -> *const c_char {
    if let TextState::Split(_, right) = &this.text {
        output_str(right)
    } else {
        output_str("")
    }
}

/// Accesses the centered text. If the text isn't centered, an empty string is
/// returned instead.
#[no_mangle]
pub extern "C" fn TextComponentState_center(this: &TextComponentState) -> *const c_char {
    if let TextState::Center(center) = &this.text {
        output_str(center)
    } else {
        output_str("")
    }
}

/// Returns whether the text is split up into a left and right part.
#[no_mangle]
pub extern "C" fn TextComponentState_is_split(this: &TextComponentState) -> bool {
    if let TextState::Split(_, _) = this.text {
        true
    } else {
        false
    }
}
