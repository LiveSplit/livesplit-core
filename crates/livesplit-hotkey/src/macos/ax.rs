#![allow(dead_code)]

use super::cf::{DictionaryRef, StringRef};

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    pub static kAXTrustedCheckOptionPrompt: StringRef;

    pub fn AXIsProcessTrustedWithOptions(options: DictionaryRef) -> bool;
}
