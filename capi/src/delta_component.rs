use livesplit_core::component::delta::Component as DeltaComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};
use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
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
    layout_settings: *const GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        acc_mut(&this)
            .state(acc(&timer), acc(&layout_settings))
            .write_json(o)
            .unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_state(
    this: *mut DeltaComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedDeltaComponentState {
    alloc(acc_mut(&this).state(acc(&timer), acc(&layout_settings)))
}
