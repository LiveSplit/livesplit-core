use super::{
    Owned,
    ax::{AXIsProcessTrustedWithOptions, kAXTrustedCheckOptionPrompt},
    cf::{
        CFDictionaryCreate, kCFAllocatorDefault, kCFBooleanTrue, kCFTypeDictionaryKeyCallBacks,
        kCFTypeDictionaryValueCallBacks,
    },
};
use std::ffi::c_void;

pub fn request() -> bool {
    unsafe {
        let keys = [kAXTrustedCheckOptionPrompt as *const c_void];
        let values = [kCFBooleanTrue as *const c_void];

        let options = CFDictionaryCreate(
            kCFAllocatorDefault,
            keys.as_ptr(),
            values.as_ptr(),
            1,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );

        let options = Owned(options);
        AXIsProcessTrustedWithOptions(options.0)
    }
}
