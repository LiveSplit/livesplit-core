//! A Component provides information about a run in a way that is easy to
//! visualize. This type can store any of the components provided by this crate.

use livesplit_core::Component;
use super::own_drop;

/// type
pub type OwnedComponent = *mut Component;

/// drop
#[no_mangle]
pub unsafe extern "C" fn Component_drop(this: OwnedComponent) {
    own_drop(this);
}
