//! Native permission queries and requests for CoreGraphics keyboard event taps.
//!
//! Selection and interaction policy are pure; this backend owns Foundation/AX
//! calls and optional CoreGraphics symbols. Permission checks run only during
//! hook construction or explicit access actions, never on keyboard callbacks.

use crate::{
    ConsumePreference, Permission, Result,
    macos::{
        Error, Owned,
        ax::{AXIsProcessTrustedWithOptions, kAXTrustedCheckOptionPrompt},
        cf::{
            CFDictionaryCreate, kCFAllocatorDefault, kCFBooleanTrue, kCFTypeDictionaryKeyCallBacks,
            kCFTypeDictionaryValueCallBacks,
        },
    },
    permission::macos::{Backend, Interaction, authorize},
};
use core::ptr::null_mut;
use std::ffi::c_void;

mod listen;
use listen::ListenAccess;

struct Native {
    listen: Option<ListenAccess>,
}

impl Backend for Native {
    fn input_monitoring_supported(&self) -> bool {
        self.listen.is_some()
    }

    fn access(&self, permission: Permission, interaction: Interaction) -> Result<bool> {
        match permission {
            Permission::InputMonitoring => Ok(self
                .listen
                .as_ref()
                .expect("Input Monitoring is selected only when its API is available")
                .access(interaction)),
            Permission::Accessibility => accessibility(interaction),
        }
    }
}

/// Read the selected grant without creating native hook resources or prompting.
pub fn check(consume: ConsumePreference) -> Result<()> {
    authorize(
        consume,
        Interaction::Check,
        &Native {
            listen: ListenAccess::load(),
        },
    )
}

/// Admit one native access request; approval may become visible only on a later
/// explicit retry. Never hold a hook lock while invoking this operation.
pub fn request(consume: ConsumePreference) -> Result<()> {
    authorize(
        consume,
        Interaction::Request,
        &Native {
            listen: ListenAccess::load(),
        },
    )
}

fn accessibility(interaction: Interaction) -> Result<bool> {
    if interaction == Interaction::Check {
        // SAFETY: NULL is the documented absence of options. Without the prompt
        // option this only reads trust; it cannot show the Accessibility prompt.
        return Ok(unsafe { AXIsProcessTrustedWithOptions(null_mut()) } != 0);
    }

    // SAFETY: All CF globals are immutable framework constants. The typed CF
    // callbacks retain the key/value; the owned dictionary is released after AX
    // reads it. The request API is reached only after explicit caller admission.
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
        if options.is_null() {
            return Err(Error::CouldntCreatePermissionOptions.into());
        }
        let options = Owned(options);
        Ok(AXIsProcessTrustedWithOptions(options.0) != 0)
    }
}
