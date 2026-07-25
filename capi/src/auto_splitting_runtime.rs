//! With an Auto Splitting Runtime, the runner can use an Auto Splitter to
//! automatically control the timer on systems that are supported.

use super::str;
use crate::shared_timer::OwnedSharedTimer;
use std::{os::raw::c_char, path::PathBuf};

#[cfg(feature = "auto-splitting")]
type AutoSplittingRuntime = livesplit_core::auto_splitting::Runtime<livesplit_core::SharedTimer>;

#[cfg(not(feature = "auto-splitting"))]
use livesplit_core::SharedTimer;

#[cfg(not(feature = "auto-splitting"))]
#[expect(missing_docs)]
pub struct AutoSplittingRuntime;

#[expect(warnings)]
#[cfg(not(feature = "auto-splitting"))]
impl AutoSplittingRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn unload(&self) -> Result<(), ()> {
        Err(())
    }

    pub fn load(&self, _: SharedTimer) -> Result<Option<PathBuf>, ()> {
        Err(())
    }

    pub fn load_from_path(&self, _: SharedTimer, _: PathBuf) -> Result<(), ()> {
        Err(())
    }

    pub fn store_settings(&self) {}
}

/// type
pub type OwnedAutoSplittingRuntime = Box<AutoSplittingRuntime>;
/// type
pub type NullableOwnedAutoSplittingRuntime = Option<OwnedAutoSplittingRuntime>;

/// Creates a new Auto Splitting Runtime.
#[unsafe(no_mangle)]
pub extern "C" fn AutoSplittingRuntime_new() -> OwnedAutoSplittingRuntime {
    Box::new(AutoSplittingRuntime::new())
}

/// Attempts to load an auto splitter. Returns true if successful.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AutoSplittingRuntime_load(
    this: &AutoSplittingRuntime,
    path: *const c_char,
    shared_timer: OwnedSharedTimer,
) -> bool {
    // SAFETY: The caller guarantees that `path` is valid.
    this.load_from_path(*shared_timer, PathBuf::from(unsafe { str(path) }))
        .is_ok()
}

/// Attempts to load the auto splitter configured in the timer's run. Returns
/// true if successful.
#[unsafe(no_mangle)]
pub extern "C" fn AutoSplittingRuntime_load_from_timer(
    this: &AutoSplittingRuntime,
    shared_timer: OwnedSharedTimer,
) -> bool {
    this.load(*shared_timer).is_ok()
}

/// Stores the loaded auto splitter's path and settings in the timer's run.
#[unsafe(no_mangle)]
pub extern "C" fn AutoSplittingRuntime_store_settings(this: &AutoSplittingRuntime) {
    this.store_settings();
}

/// Attempts to unload the auto splitter. Returns true if successful.
#[unsafe(no_mangle)]
pub extern "C" fn AutoSplittingRuntime_unload(this: &AutoSplittingRuntime) -> bool {
    this.unload().is_ok()
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn AutoSplittingRuntime_drop(this: OwnedAutoSplittingRuntime) {
    drop(this);
}
