#![allow(dead_code)]

use super::cf::StringRef;
use std::os::raw::{c_ulong, c_void};

mod opaque {
    pub enum TISInputSource {}
    pub enum UCKeyboardLayout {}
}

pub type TISInputSourceRef = *mut opaque::TISInputSource;

pub type OptionBits = u32;
pub type UniCharCount = c_ulong;
pub type UniChar = u16;
pub type OSStatus = i32;

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct UCKeyTranslateBits: OptionBits {
        const NO_DEAD_KEYS_BIT = 0;
    }
}

#[repr(u16)]
#[non_exhaustive]
pub enum UCKeyAction {
    Down = 0,
    Up = 1,
    AutoKey = 2,
    Display = 3,
}

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    pub static kTISPropertyUnicodeKeyLayoutData: StringRef;
    pub fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
    pub fn TISCopyCurrentKeyboardLayoutInputSource() -> TISInputSourceRef;
    pub fn TISGetInputSourceProperty(
        input_source: TISInputSourceRef,
        property_key: StringRef,
    ) -> *mut c_void;
    pub fn UCKeyTranslate(
        key_layout_ptr: *const opaque::UCKeyboardLayout,
        virtual_key_code: u16,
        key_action: u16,
        modifier_key_state: u32,
        keyboard_type: u32,
        key_translate_options: OptionBits,
        dead_key_state: *mut u32,
        max_string_length: UniCharCount,
        actual_string_length: *mut UniCharCount,
        unicode_string: *mut UniChar,
    ) -> OSStatus;
    pub fn LMGetKbdType() -> u8;
}
