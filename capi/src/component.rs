//! A Component provides information about a run in a way that is easy to
//! visualize. This type can store any of the components provided by this crate.

use livesplit_core::Component;

/// type
pub type OwnedComponent = Box<Component>;

/// drop
#[no_mangle]
pub extern "C" fn Component_drop(this: OwnedComponent) {
    drop(this);
}
