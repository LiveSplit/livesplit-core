//! An iterator iterating over all the Run Metadata variables and their values
//! that have been specified.

use super::{acc_mut, own_drop, RUN_METADATA_VARIABLE};
use std::ptr;
use livesplit_core::indexmap;
use run_metadata_variable::{NullableRunMetadataVariable, RunMetadataVariable};

/// type
pub type RunMetadataVariablesIter = indexmap::map::Iter<'static, String, String>;
/// type
pub type OwnedRunMetadataVariablesIter = *mut RunMetadataVariablesIter;

/// drop
#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariablesIter_drop(this: OwnedRunMetadataVariablesIter) {
    own_drop(this);
}

/// Accesses the next Run Metadata variable. Returns <NULL> if there are no more
/// variables.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariablesIter_next(
    this: *mut RunMetadataVariablesIter,
) -> *const NullableRunMetadataVariable {
    if let Some((name, value)) = acc_mut(this).next() {
        RUN_METADATA_VARIABLE.with(|output| {
            output.set((name, value));
            output.as_ptr() as *const RunMetadataVariable
        })
    } else {
        ptr::null()
    }
}
