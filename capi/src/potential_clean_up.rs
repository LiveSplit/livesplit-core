//! Describes a potential clean up that could be applied. You can query a
//! message describing the details of this potential clean up. A potential clean
//! up can then be turned into an actual clean up in order to apply it to the
//! Run.

use livesplit_core::run::editor::cleaning::PotentialCleanUp;
use super::{acc, output_vec, own_drop};
use std::os::raw::c_char;
use std::io::Write;

/// type
pub type OwnedPotentialCleanUp = *mut PotentialCleanUp<'static>;
/// type
pub type NullableOwnedPotentialCleanUp = OwnedPotentialCleanUp;

/// drop
#[no_mangle]
pub unsafe extern "C" fn PotentialCleanUp_drop(this: OwnedPotentialCleanUp) {
    own_drop(this);
}

/// Accesses the message describing the potential clean up that can be applied
/// to a Run.
#[no_mangle]
pub unsafe extern "C" fn PotentialCleanUp_message(
    this: *const PotentialCleanUp<'static>,
) -> *const c_char {
    output_vec(|s| write!(s, "{}", acc(this)).unwrap())
}
