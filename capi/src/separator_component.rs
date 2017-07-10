use livesplit_core::component::separator::Component as SeparatorComponent;
use super::{alloc, own, own_drop};
use component::OwnedComponent;

pub type OwnedSeparatorComponent = *mut SeparatorComponent;

#[no_mangle]
pub unsafe extern "C" fn SeparatorComponent_new() -> OwnedSeparatorComponent {
    alloc(SeparatorComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn SeparatorComponent_drop(this: OwnedSeparatorComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SeparatorComponent_into_generic(
    this: OwnedSeparatorComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}
