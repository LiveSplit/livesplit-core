//! The Timer Component is a component that shows the total time of the current
//! attempt as a digital clock. The color of the time shown is based on a how
//! well the current attempt is doing compared to the chosen comparison.

use super::{acc, alloc, output_vec, own, own_drop, Json};
use component::OwnedComponent;
use livesplit_core::component::timer::Component as TimerComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};
use timer_component_state::OwnedTimerComponentState;

/// type
pub type OwnedTimerComponent = *mut TimerComponent;

/// Creates a new Timer Component.
#[no_mangle]
pub unsafe extern "C" fn TimerComponent_new() -> OwnedTimerComponent {
    alloc(TimerComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn TimerComponent_drop(this: OwnedTimerComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn TimerComponent_into_generic(this: OwnedTimerComponent) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
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

/// Calculates the component's state based on the timer and the layout
/// settings provided.
#[no_mangle]
pub unsafe extern "C" fn TimerComponent_state(
    this: *const TimerComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedTimerComponentState {
    alloc(acc(this).state(acc(timer), acc(layout_settings)))
}
