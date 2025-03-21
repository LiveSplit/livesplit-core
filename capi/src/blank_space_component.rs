//! The Blank Space Component is simply an empty component that doesn't show
//! anything other than a background. It mostly serves as padding between other
//! components.

use super::{output_vec, Json};
use crate::blank_space_component_state::OwnedBlankSpaceComponentState;
use crate::component::OwnedComponent;
use livesplit_core::component::blank_space::Component as BlankSpaceComponent;

/// type
pub type OwnedBlankSpaceComponent = Box<BlankSpaceComponent>;

/// Creates a new Blank Space Component.
#[unsafe(no_mangle)]
pub extern "C" fn BlankSpaceComponent_new() -> OwnedBlankSpaceComponent {
    Box::new(BlankSpaceComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn BlankSpaceComponent_drop(this: OwnedBlankSpaceComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn BlankSpaceComponent_into_generic(
    this: OwnedBlankSpaceComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn BlankSpaceComponent_state_as_json(this: &mut BlankSpaceComponent) -> Json {
    output_vec(|o| {
        this.state().write_json(o).unwrap();
    })
}

/// Calculates the component's state.
#[unsafe(no_mangle)]
pub extern "C" fn BlankSpaceComponent_state(
    this: &mut BlankSpaceComponent,
) -> OwnedBlankSpaceComponentState {
    Box::new(this.state())
}
