//! The Timer Component is a component that shows the total time of the current
//! attempt as a digital clock. The color of the time shown is based on a how
//! well the current attempt is doing compared to the chosen comparison.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::timer_component_state::OwnedTimerComponentState;
use livesplit_core::component::timer::Component as TimerComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};

/// type
pub type OwnedTimerComponent = Box<TimerComponent>;

/// Creates a new Timer Component.
#[no_mangle]
pub extern "C" fn TimerComponent_new() -> OwnedTimerComponent {
    Box::new(TimerComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn TimerComponent_drop(this: OwnedTimerComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn TimerComponent_into_generic(this: OwnedTimerComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn TimerComponent_state_as_json(
    this: &TimerComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        this.state(timer, layout_settings).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer and the layout
/// settings provided.
#[no_mangle]
pub extern "C" fn TimerComponent_state(
    this: &TimerComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> OwnedTimerComponentState {
    Box::new(this.state(timer, layout_settings))
}
