use super::{
    ax::{AXIsProcessTrusted, AXIsProcessTrustedWithOptions, kAXTrustedCheckOptionPrompt}, 
    cf::{kCFAllocatorDefault, kCFBooleanTrue, kCFTypeDictionaryKeyCallBacks, 
        kCFTypeDictionaryValueCallBacks, CFDictionaryCreate},
    Owned
};
use std::ffi::c_void;

pub fn granted() -> bool {
    unsafe {
        AXIsProcessTrusted()
    }
}

pub fn request() -> bool {
    unsafe {
        let keys: [*const c_void; 1] = [
            kAXTrustedCheckOptionPrompt as *const c_void
        ];
        let values: [*const c_void; 1] = [
            kCFBooleanTrue as *const c_void
        ];

        let options = CFDictionaryCreate(
            kCFAllocatorDefault,
            keys.as_ptr(),
            values.as_ptr(),
            1,
            &kCFTypeDictionaryKeyCallBacks, 
            &kCFTypeDictionaryValueCallBacks);

        let options = Owned(options);
        AXIsProcessTrustedWithOptions(options.0)
    }
}