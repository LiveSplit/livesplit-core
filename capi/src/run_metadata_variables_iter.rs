use super::{own_drop, acc_mut, RUN_METADATA_VARIABLE};
use std::ptr;
use std::collections::btree_map;
use run_metadata_variable::RunMetadataVariable;

pub type RunMetadataVariablesIter = btree_map::Iter<'static, String, String>;
pub type OwnedRunMetadataVariablesIter = *mut RunMetadataVariablesIter;

#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariablesIter_drop(this: OwnedRunMetadataVariablesIter) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariablesIter_next(this: *mut RunMetadataVariablesIter)
                                                       -> *const RunMetadataVariable {
    if let Some((name, value)) = acc_mut(this).next() {
        RUN_METADATA_VARIABLE.with(|output| {
                                       output.set((name, value));
                                       output.as_ptr() as *const RunMetadataVariable
                                   })
    } else {
        ptr::null()
    }
}
