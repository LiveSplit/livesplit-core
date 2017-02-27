use livesplit_core::RunMetadata;
use super::{acc, output_str};
use libc::c_char;

pub type OwnedRunMetadata = *mut RunMetadata;

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_run_id(this: *const RunMetadata) -> *const c_char {
    output_str(acc(this).run_id())
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_platform_name(this: *const RunMetadata) -> *const c_char {
    output_str(acc(this).platform_name())
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_uses_emulator(this: *const RunMetadata) -> bool {
    acc(this).uses_emulator()
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_region_name(this: *const RunMetadata) -> *const c_char {
    output_str(acc(this).region_name())
}
