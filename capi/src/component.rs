use livesplit_core::Component;
use super::own_drop;

pub type OwnedComponent = *mut Component;

#[no_mangle]
pub unsafe extern "C" fn Component_drop(this: OwnedComponent) {
    own_drop(this);
}
