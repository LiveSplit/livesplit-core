//! The state object describes the information to visualize for this component.

use livesplit_core::component::separator::State as SeparatorComponentState;

/// type
pub type OwnedSeparatorComponentState = Box<SeparatorComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn SeparatorComponentState_drop(this: OwnedSeparatorComponentState) {
    drop(this);
}
