#![allow(dead_code)]

use super::cf::{DictionaryRef, StringRef};
use std::ffi::c_uchar;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    pub static kAXTrustedCheckOptionPrompt: StringRef;

    // Apple's Boolean is an unsigned byte, distinct from C's bool. Convert
    // the native value with != 0 at the permission boundary.
    pub fn AXIsProcessTrustedWithOptions(options: DictionaryRef) -> c_uchar;
}
