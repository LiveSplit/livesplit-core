//! The Run Metadata stores additional information about a run, like the
//! platform and region of the game. All of this information is optional.

use livesplit_core::RunMetadata;
use super::{acc, alloc, output_str};
use libc::c_char;
use run_metadata_variables_iter::OwnedRunMetadataVariablesIter;

/// type
pub type OwnedRunMetadata = *mut RunMetadata;

/// Accesses the speedrun.com Run ID of the run. This Run ID specify which
/// Record on speedrun.com this run is associated with. This should be
/// changed once the Personal Best doesn't match up with that record
/// anymore. This may be empty if there's no association.
#[no_mangle]
pub unsafe extern "C" fn RunMetadata_run_id(this: *const RunMetadata) -> *const c_char {
    output_str(acc(this).run_id())
}

/// Accesses the name of the platform this game is run on. This may be empty
/// if it's not specified.
#[no_mangle]
pub unsafe extern "C" fn RunMetadata_platform_name(this: *const RunMetadata) -> *const c_char {
    output_str(acc(this).platform_name())
}

/// Returns <TRUE> if this speedrun is done on an emulator. However <FALSE>
/// may also indicate that this information is simply not known.
#[no_mangle]
pub unsafe extern "C" fn RunMetadata_uses_emulator(this: *const RunMetadata) -> bool {
    acc(this).uses_emulator()
}

/// Accesses the name of the region this game is from. This may be empty if
/// it's not specified.
#[no_mangle]
pub unsafe extern "C" fn RunMetadata_region_name(this: *const RunMetadata) -> *const c_char {
    output_str(acc(this).region_name())
}

/// Returns an iterator iterating over all the variables and their values
/// that have been specified.
#[no_mangle]
pub unsafe extern "C" fn RunMetadata_variables(
    this: *const RunMetadata,
) -> OwnedRunMetadataVariablesIter {
    alloc(acc(this).variables())
}
