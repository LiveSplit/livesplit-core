//! Optional Input Monitoring entry points, available starting with macOS 10.15.
//!
//! Resolve the native capability rather than raising the library's deployment
//! target. CoreGraphics is already linked for event taps and remains loaded for
//! the process lifetime; no loader handle or global mutable cache is retained.
//!
//! FIXME: This dynamic lookup preserves compatibility with macOS before 10.15.
//! Once the library's minimum supported macOS version is 10.15 or newer, link
//! these functions directly and remove the legacy Accessibility fallback for
//! listening taps. Consuming taps still require the Accessibility path.

use crate::permission::macos::Interaction;
use core::mem;
use std::ffi::{c_char, c_void};

type Access = unsafe extern "C" fn() -> bool;

// Darwin's dlfcn.h defines RTLD_DEFAULT as -2. It is a loader search token,
// never a Rust pointer to dereference. Lookups occur only at explicit access
// boundaries; there is no per-event lookup or recurring permission polling.
const RTLD_DEFAULT: *mut c_void = (-2_isize) as *mut c_void;

#[link(name = "System")]
unsafe extern "C" {
    /// # Safety
    /// Supply a valid Darwin loader handle/search token and a live terminated
    /// symbol name. A returned address must be used only with its actual ABI.
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// A validated pair of process-lifetime CoreGraphics entry points.
pub struct ListenAccess {
    check: Access,
    request: Access,
}

impl ListenAccess {
    /// Return None on systems without both entry points; perform no access query.
    pub fn load() -> Option<Self> {
        // SAFETY: These exact CoreGraphics symbols have the declared C ABI:
        // no arguments, C bool result. dlsym accepts the RTLD_DEFAULT token and
        // nul-terminated names; reject absent symbols before converting. The
        // linked framework cannot unload while these function pointers are used.
        unsafe {
            let check = dlsym(RTLD_DEFAULT, c"CGPreflightListenEventAccess".as_ptr());
            let request = dlsym(RTLD_DEFAULT, c"CGRequestListenEventAccess".as_ptr());

            if check.is_null() || request.is_null() {
                return None;
            }

            Some(Self {
                check: mem::transmute::<*mut c_void, Access>(check),
                request: mem::transmute::<*mut c_void, Access>(request),
            })
        }
    }

    /// Invoke the selected entry point without retaining any caller state.
    pub fn access(&self, interaction: Interaction) -> bool {
        let call = match interaction {
            Interaction::Check => self.check,
            Interaction::Request => self.request,
        };
        // SAFETY: load validated the symbol's presence and ABI; the framework
        // remains linked. Neither function accepts pointers or borrowed state.
        unsafe { call() }
    }
}
