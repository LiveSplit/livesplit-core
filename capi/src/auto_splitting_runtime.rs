//! With an Auto Splitting Runtime, the runner can use an Auto Splitter to
//! automatically control the timer on systems that are supported.

use super::str;
use crate::shared_timer::OwnedSharedTimer;
use std::os::raw::c_char;
use std::path::PathBuf;

#[cfg(feature = "auto-splitting")]
use livesplit_core::auto_splitting::Runtime as AutoSplittingRuntime;

#[cfg(not(feature = "auto-splitting"))]
use livesplit_core::SharedTimer;

#[cfg(not(feature = "auto-splitting"))]
#[allow(missing_docs)]
pub struct AutoSplittingRuntime;

#[allow(missing_docs)]
#[cfg(not(feature = "auto-splitting"))]
impl AutoSplittingRuntime {
    pub fn new(_: SharedTimer) -> Self {
        Self
    }

    pub fn unload_script(&self) -> Result<(), ()> {
        Err(())
    }

    pub fn load_script(&self, _: PathBuf) -> Result<(), ()> {
        Err(())
    }
}

/// type
pub type OwnedAutoSplittingRuntime = Box<AutoSplittingRuntime>;
/// type
pub type NullableOwnedAutoSplittingRuntime = Option<OwnedAutoSplittingRuntime>;

/// Creates a new Auto Splitting Runtime for a Timer.
#[no_mangle]
pub extern "C" fn AutoSplittingRuntime_new(
    shared_timer: OwnedSharedTimer,
) -> OwnedAutoSplittingRuntime {
    Box::new(AutoSplittingRuntime::new(*shared_timer))
}

/// Attempts to load an auto splitter. Returns true if successful.
#[no_mangle]
pub unsafe extern "C" fn AutoSplittingRuntime_load_script(
    this: &AutoSplittingRuntime,
    path: *const c_char,
) -> bool {
    let path = str(path);
    if !path.is_empty() {
        this.load_script(PathBuf::from(path)).is_ok()
    } else {
        false
    }
}

/// Attempts to unload the auto splitter. Returns true if successful.
#[no_mangle]
pub extern "C" fn AutoSplittingRuntime_unload_script(this: &AutoSplittingRuntime) -> bool {
    this.unload_script().is_ok()
}

/// drop
#[no_mangle]
pub extern "C" fn AutoSplittingRuntime_drop(this: OwnedAutoSplittingRuntime) {
    drop(this);
}
