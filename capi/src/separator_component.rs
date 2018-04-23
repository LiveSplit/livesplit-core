//! The Separator Component is a simple component that only serves to render
//! separators between components.

use super::{alloc, own, own_drop};
use component::OwnedComponent;
use livesplit_core::component::separator::Component as SeparatorComponent;

/// type
pub type OwnedSeparatorComponent = *mut SeparatorComponent;

/// Creates a new Separator Component.
#[no_mangle]
pub unsafe extern "C" fn SeparatorComponent_new() -> OwnedSeparatorComponent {
    alloc(SeparatorComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn SeparatorComponent_drop(this: OwnedSeparatorComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn SeparatorComponent_into_generic(
    this: OwnedSeparatorComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}
