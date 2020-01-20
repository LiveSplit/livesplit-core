//! The Separator Component is a simple component that only serves to render
//! separators between components.

use crate::component::OwnedComponent;
use crate::separator_component_state::OwnedSeparatorComponentState;
use livesplit_core::component::separator::Component as SeparatorComponent;
use livesplit_core::Timer;

/// type
pub type OwnedSeparatorComponent = Box<SeparatorComponent>;

/// Creates a new Separator Component.
#[no_mangle]
pub extern "C" fn SeparatorComponent_new() -> OwnedSeparatorComponent {
    Box::new(SeparatorComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn SeparatorComponent_drop(this: OwnedSeparatorComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn SeparatorComponent_into_generic(this: OwnedSeparatorComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub extern "C" fn SeparatorComponent_state(
    this: &mut SeparatorComponent,
    timer: &Timer,
) -> OwnedSeparatorComponentState {
    Box::new(this.state(timer))
}
