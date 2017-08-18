use livesplit_core::RunMetadata;
use super::{acc, alloc, output_str};
use libc::c_char;
use run_metadata_variables_iter::OwnedRunMetadataVariablesIter;

pub type OwnedRunMetadata = *mut RunMetadata;

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_run_id(this: *const RunMetadata) -> *const c_char {
    output_str(&acc(this).run_id)
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_platform_name(this: *const RunMetadata) -> *const c_char {
    output_str(&acc(this).platform_name)
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_uses_emulator(this: *const RunMetadata) -> bool {
    acc(this).uses_emulator
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_region_name(this: *const RunMetadata) -> *const c_char {
    output_str(&acc(this).region_name)
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_variables(
    this: *const RunMetadata,
) -> OwnedRunMetadataVariablesIter {
    alloc(acc(this).variables())
}
