//! Describes a potential clean up that could be applied. You can query a
//! message describing the details of this potential clean up. A potential clean
//! up can then be turned into an actual clean up in order to apply it to the
//! Run.

use super::output_vec;
use livesplit_core::run::editor::cleaning::PotentialCleanUp;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedPotentialCleanUp = Box<PotentialCleanUp<'static>>;
/// type
pub type NullableOwnedPotentialCleanUp = Option<OwnedPotentialCleanUp>;

/// drop
#[no_mangle]
pub extern "C" fn PotentialCleanUp_drop(this: OwnedPotentialCleanUp) {
    drop(this);
}

/// Accesses the message describing the potential clean up that can be applied
/// to a Run.
#[no_mangle]
pub extern "C" fn PotentialCleanUp_message(this: &PotentialCleanUp<'static>) -> *const c_char {
    output_vec(|s| write!(s, "{}", this).unwrap())
}
