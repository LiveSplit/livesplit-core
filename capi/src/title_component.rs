use livesplit_core::component::title::Component as TitleComponent;
use livesplit_core::Timer;
use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use title_component_state::OwnedTitleComponentState;
use component::OwnedComponent;

pub type OwnedTitleComponent = *mut TitleComponent;

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_new() -> OwnedTitleComponent {
    alloc(TitleComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_drop(this: OwnedTitleComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_into_generic(this: OwnedTitleComponent) -> OwnedComponent {
    alloc(own(this).into())
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_state_as_json(
    this: *mut TitleComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(&this).state(acc(&timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_state(
    this: *mut TitleComponent,
    timer: *const Timer,
) -> OwnedTitleComponentState {
    alloc(acc_mut(&this).state(acc(&timer)))
}
