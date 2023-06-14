#![allow(dead_code)]

use super::cf::{DictionaryRef, StringRef};

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {

    pub static kAXTrustedCheckOptionPrompt: StringRef;

    pub fn AXIsProcessTrusted() -> bool;
    pub fn AXIsProcessTrustedWithOptions(options: DictionaryRef) -> bool;

}
