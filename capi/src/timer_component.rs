use livesplit_core::component::timer::Component as TimerComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};
use super::{acc, alloc, output_vec, own, own_drop, Json};
use timer_component_state::OwnedTimerComponentState;
use component::OwnedComponent;

pub type OwnedTimerComponent = *mut TimerComponent;

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_new() -> OwnedTimerComponent {
    alloc(TimerComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_drop(this: OwnedTimerComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_into_generic(this: OwnedTimerComponent) -> OwnedComponent {
    alloc(own(this).into())
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_state_as_json(
    this: *const TimerComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        acc(this)
            .state(acc(timer), acc(layout_settings))
            .write_json(o)
            .unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_state(
    this: *const TimerComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedTimerComponentState {
    alloc(acc(this).state(acc(timer), acc(layout_settings)))
}
