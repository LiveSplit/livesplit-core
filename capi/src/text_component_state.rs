use livesplit_core::component::text::State as TextComponentState;
use livesplit_core::component::text::Text;
use super::{own_drop, acc, output_str};
use libc::c_char;

pub type OwnedTextComponentState = *mut TextComponentState;

#[no_mangle]
pub unsafe extern "C" fn TextComponentState_drop(this: OwnedTextComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TextComponentState_left(this: *const TextComponentState) -> *const c_char {
    if let &TextComponentState(Text::Split(ref left, _)) = acc(this) {
        output_str(left)
    } else {
        output_str("")
    }
}

#[no_mangle]
pub unsafe extern "C" fn TextComponentState_right(this: *const TextComponentState)
                                                  -> *const c_char {
    if let &TextComponentState(Text::Split(_, ref right)) = acc(this) {
        output_str(right)
    } else {
        output_str("")
    }
}

#[no_mangle]
pub unsafe extern "C" fn TextComponentState_center(this: *const TextComponentState)
                                                   -> *const c_char {
    if let &TextComponentState(Text::Center(ref center)) = acc(this) {
        output_str(center)
    } else {
        output_str("")
    }
}

#[no_mangle]
pub unsafe extern "C" fn TextComponentState_is_split(this: *const TextComponentState) -> bool {
    if let &TextComponentState(Text::Split(_, _)) = acc(this) {
        true
    } else {
        false
    }
}
