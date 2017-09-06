use livesplit_core::component::detailed_timer::Component as DetailedTimerComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};
use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use detailed_timer_component_state::OwnedDetailedTimerComponentState;
use component::OwnedComponent;

pub type OwnedDetailedTimerComponent = *mut DetailedTimerComponent;

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponent_new() -> OwnedDetailedTimerComponent {
    alloc(DetailedTimerComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponent_drop(this: OwnedDetailedTimerComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponent_into_generic(
    this: OwnedDetailedTimerComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponent_state_as_json(
    this: *mut DetailedTimerComponent,
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
pub unsafe extern "C" fn DetailedTimerComponent_state(
    this: *mut DetailedTimerComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedDetailedTimerComponentState {
    alloc(acc_mut(&this).state(acc(&timer), acc(&layout_settings)))
}
