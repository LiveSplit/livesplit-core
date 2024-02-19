//! With an Auto Splitting Runtime, the runner can use an Auto Splitter to
//! automatically control the timer on systems that are supported.

use super::str;
use crate::shared_timer::OwnedSharedTimer;
use std::{os::raw::c_char, path::PathBuf};

#[cfg(feature = "auto-splitting")]
use livesplit_core::auto_splitting::Runtime as AutoSplittingRuntime;

#[cfg(not(feature = "auto-splitting"))]
use livesplit_core::SharedTimer;

#[cfg(not(feature = "auto-splitting"))]
#[allow(missing_docs)]
pub struct AutoSplittingRuntime;

#[allow(warnings)]
#[cfg(not(feature = "auto-splitting"))]
impl AutoSplittingRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn unload(&self) -> Result<(), ()> {
        Err(())
    }

    pub fn load(&self, _: PathBuf, _: SharedTimer) -> Result<(), ()> {
        Err(())
    }
}

/// type
pub type OwnedAutoSplittingRuntime = Box<AutoSplittingRuntime>;
/// type
pub type NullableOwnedAutoSplittingRuntime = Option<OwnedAutoSplittingRuntime>;

/// Creates a new Auto Splitting Runtime.
#[no_mangle]
pub extern "C" fn AutoSplittingRuntime_new() -> OwnedAutoSplittingRuntime {
    Box::new(AutoSplittingRuntime::new())
}

/// Attempts to load an auto splitter. Returns true if successful.
#[no_mangle]
pub unsafe extern "C" fn AutoSplittingRuntime_load(
    this: &AutoSplittingRuntime,
    path: *const c_char,
    shared_timer: OwnedSharedTimer,
) -> bool {
    this.load(PathBuf::from(str(path)), *shared_timer).is_ok()
}

/// Attempts to unload the auto splitter. Returns true if successful.
#[no_mangle]
pub extern "C" fn AutoSplittingRuntime_unload(this: &AutoSplittingRuntime) -> bool {
    this.unload().is_ok()
}

/// drop
#[no_mangle]
pub extern "C" fn AutoSplittingRuntime_drop(this: OwnedAutoSplittingRuntime) {
    drop(this);
}
