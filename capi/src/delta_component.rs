use livesplit_core::component::delta::Component as DeltaComponent;
use livesplit_core::Timer;
use super::{Json, alloc, own, own_drop, acc, output_vec, acc_mut};
use delta_component_state::OwnedDeltaComponentState;
use component::OwnedComponent;

pub type OwnedDeltaComponent = *mut DeltaComponent;

#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_new() -> OwnedDeltaComponent {
    alloc(DeltaComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_drop(this: OwnedDeltaComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_into_generic(this: OwnedDeltaComponent) -> OwnedComponent {
    alloc(own(this).into())
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_state_as_json(
    this: *mut DeltaComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_state(
    this: *mut DeltaComponent,
    timer: *const Timer,
) -> OwnedDeltaComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
