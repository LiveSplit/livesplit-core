//! The Detailed Timer Component is a component that shows two timers, one for
//! the total time of the current attempt and one showing the time of just the
//! current segment. Other information, like segment times of up to two
//! comparisons, the segment icon, and the segment's name, can also be shown.

use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use component::OwnedComponent;
use detailed_timer_component_state::OwnedDetailedTimerComponentState;
use livesplit_core::component::detailed_timer::Component as DetailedTimerComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};

/// type
pub type OwnedDetailedTimerComponent = *mut DetailedTimerComponent;

/// Creates a new Detailed Timer Component.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponent_new() -> OwnedDetailedTimerComponent {
    alloc(DetailedTimerComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponent_drop(this: OwnedDetailedTimerComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponent_into_generic(
    this: OwnedDetailedTimerComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponent_state_as_json(
    this: *mut DetailedTimerComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        acc_mut(this)
            .state(acc(timer), acc(layout_settings))
            .write_json(o)
            .unwrap();
    })
}

/// Calculates the component's state based on the timer and layout settings
/// provided.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponent_state(
    this: *mut DetailedTimerComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedDetailedTimerComponentState {
    alloc(acc_mut(this).state(acc(timer), acc(layout_settings)))
}
