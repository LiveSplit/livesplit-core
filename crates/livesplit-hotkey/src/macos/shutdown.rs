//! Persistent stop requests for the macOS tap's private run loop.
//!
//! A signaled source survives the interval before CFRunLoopRun starts, unlike
//! CFRunLoopStop. The worker schedules this source before publishing its handle
//! and removes it before exiting. The control handle retains both native objects
//! independently, so it remains safe if the worker has already stopped.

use crate::macos::{Error, Owned, RunLoop, cf};
use std::{ffi::c_void, ptr::null_mut};

pub struct Shutdown {
    pub event_loop: RunLoop,
    pub source: Owned<cf::RunLoopSource>,
}

impl Shutdown {
    pub fn new() -> Result<Self, Error> {
        let event_loop = RunLoop::current().ok_or(Error::CouldntGetCurrentRunLoop)?;
        let mut context = cf::RunLoopSourceContext {
            version: 0,
            info: null_mut(),
            retain: None,
            release: None,
            copy_description: None,
            equal: None,
            hash: None,
            schedule: None,
            cancel: None,
            perform: Some(stop),
        };
        // SAFETY: Version-zero context matches the SDK ABI. CF copies it and
        // retains no user data. Its perform function operates on its own loop.
        let source = unsafe { cf::CFRunLoopSourceCreate(cf::kCFAllocatorDefault, 0, &mut context) };
        if source.is_null() {
            return Err(Error::CouldntCreateRunLoopSource);
        }

        Ok(Self {
            event_loop,
            source: Owned(source),
        })
    }

    pub fn request(&self) {
        // SAFETY: Both objects are retained by this handle. Signaling sources
        // and waking loops support cross-thread callers, including before run
        // starts or after the worker removes the source and exits.
        unsafe {
            cf::CFRunLoopSourceSignal(self.source.0);
            cf::CFRunLoopWakeUp(self.event_loop.0);
        }
    }
}

// SAFETY: The handle only exposes thread-safe CF source signaling and loop
// wakeup. The worker alone schedules/removes the source and executes callbacks.
unsafe impl Send for Shutdown {}

// SAFETY: Concurrent requests signal the same retained source; CF synchronizes
// those operations. No callback data or mutable Rust state is shared here.
unsafe impl Sync for Shutdown {}

/// # Safety
/// Invoked by CoreFoundation as the perform callback of a scheduled source on
/// its live run loop. The context has no user data and is never dereferenced.
unsafe extern "C" fn stop(_: *mut c_void) {
    // SAFETY: CF invokes this perform callback on the source's current loop.
    // Stopping that invocation requires no user data or cross-thread borrow.
    unsafe { cf::CFRunLoopStop(cf::CFRunLoopGetCurrent()) };
}
