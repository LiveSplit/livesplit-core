//! The state object describes the information to visualize for this component.

use super::{acc, output_str, own_drop};
use livesplit_core::component::text::State as TextComponentState;
use livesplit_core::component::text::Text;
use std::os::raw::c_char;

/// type
pub type OwnedTextComponentState = *mut TextComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn TextComponentState_drop(this: OwnedTextComponentState) {
    own_drop(this);
}

/// Accesses the left part of the text. If the text isn't split up, an empty
/// string is returned instead.
#[no_mangle]
pub unsafe extern "C" fn TextComponentState_left(this: *const TextComponentState) -> *const c_char {
    if let Text::Split(ref left, _) = acc(this).text {
        output_str(left)
    } else {
        output_str("")
    }
}

/// Accesses the right part of the text. If the text isn't split up, an empty
/// string is returned instead.
#[no_mangle]
pub unsafe extern "C" fn TextComponentState_right(
    this: *const TextComponentState,
) -> *const c_char {
    if let Text::Split(_, ref right) = acc(this).text {
        output_str(right)
    } else {
        output_str("")
    }
}

/// Accesses the centered text. If the text isn't centered, an empty string is
/// returned instead.
#[no_mangle]
pub unsafe extern "C" fn TextComponentState_center(
    this: *const TextComponentState,
) -> *const c_char {
    if let Text::Center(ref center) = acc(this).text {
        output_str(center)
    } else {
        output_str("")
    }
}

/// Returns whether the text is split up into a left and right part.
#[no_mangle]
pub unsafe extern "C" fn TextComponentState_is_split(this: *const TextComponentState) -> bool {
    if let Text::Split(_, _) = acc(this).text {
        true
    } else {
        false
    }
}
