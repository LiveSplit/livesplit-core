use super::{acc_mut, own_drop, RUN_METADATA_VARIABLE};
use std::ptr;
use livesplit_core::ordermap;
use run_metadata_variable::{NullableRunMetadataVariable, RunMetadataVariable};

pub type RunMetadataVariablesIter<'a> = ordermap::Iter<'a, String, String>;
pub type OwnedRunMetadataVariablesIter<'a> = *mut RunMetadataVariablesIter<'a>;

#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariablesIter_drop(this: OwnedRunMetadataVariablesIter) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariablesIter_next(
    this: *mut RunMetadataVariablesIter,
) -> *const NullableRunMetadataVariable {
    if let Some((name, value)) = acc_mut(&this).next() {
        RUN_METADATA_VARIABLE.with(|output| {
            output.set((name, value));
            output.as_ptr() as *const RunMetadataVariable
        })
    } else {
        ptr::null()
    }
}
