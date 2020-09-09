//! With an Auto Splitting Runtime, the runner can use an auto splitter to
//! automatically control the timer on systems that are supported.

use crate::shared_timer::OwnedSharedTimer;
use crate::{get_file, release_file};
use livesplit_core::auto_splitting::Runtime as AutoSplittingRuntime;
use std::slice;
use std::io::Read;

/// type
pub type OwnedAutoSplittingRuntime = Box<AutoSplittingRuntime>;
/// type
pub type NullableOwnedAutoSplittingRuntime = Option<OwnedAutoSplittingRuntime>;

/// Creates a new Auto Splitting Runtime for a Timer.
#[cfg(feature = "auto-splitting")]
#[no_mangle]
pub extern "C" fn AutoSplittingRuntime_new(shared_timer: OwnedSharedTimer) -> OwnedAutoSplittingRuntime {
    Box::new(AutoSplittingRuntime::new(*shared_timer))
}

/// Attempts to load an auto splitter. Returns true if successful.
#[cfg(feature = "auto-splitting")]
#[no_mangle]
pub unsafe extern "C" fn AutoSplittingRuntime_load_script(this: &AutoSplittingRuntime, buf: *const u8, len: usize) -> bool {
    this.load_script(slice::from_raw_parts(buf, len).to_vec()).is_ok()
}

/// Attempts to load an auto splitter from a given file. Returns true if
/// successful. This will not close the file descriptor/handle.
#[cfg(feature = "auto-splitting")]
#[no_mangle]
pub unsafe extern "C" fn AutoSplittingRuntime_load_script_file_handle(this: &AutoSplittingRuntime, handle: i64) -> bool {
    let mut result = false;
    let mut file = get_file(handle);
    let mut buf: Vec<u8> = Vec::new();

    if file.read_to_end(&mut buf).is_ok() {
        result = this.load_script(buf).is_ok();
    }

    release_file(file);

    result
}
/// Attempts to unload the auto splitter. Returns true if successful.
#[cfg(feature = "auto-splitting")]
#[no_mangle]
pub extern "C" fn AutoSplittingRuntime_unload_script(this: &AutoSplittingRuntime) -> bool {
    this.unload_script().is_ok()
}

/// drop
#[cfg(feature = "auto-splitting")]
#[no_mangle]
pub extern "C" fn AutoSplittingRuntime_drop(this: OwnedAutoSplittingRuntime) {
    drop(this);
}
