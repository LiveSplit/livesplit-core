//! The Alternate Timing Method Component shows the current time in the timing
//! method that is not currently being used. So if the timer is comparing
//! against Game Time, it shows the current Real Time and vice versa.

use super::{Json, output_vec};
use crate::{component::OwnedComponent, key_value_component_state::OwnedKeyValueComponentState};
use livesplit_core::{
    Lang, Timer, component::alternate_timing_method::Component as AlternateTimingMethodComponent,
};

/// type
pub type OwnedAlternateTimingMethodComponent = Box<AlternateTimingMethodComponent>;

/// Creates a new Alternate Timing Method Component.
#[unsafe(no_mangle)]
pub extern "C" fn AlternateTimingMethodComponent_new() -> OwnedAlternateTimingMethodComponent {
    Box::new(AlternateTimingMethodComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn AlternateTimingMethodComponent_drop(this: OwnedAlternateTimingMethodComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn AlternateTimingMethodComponent_into_generic(
    this: OwnedAlternateTimingMethodComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn AlternateTimingMethodComponent_state_as_json(
    this: &AlternateTimingMethodComponent,
    timer: &Timer,
    lang: Lang,
) -> Json {
    output_vec(|o| {
        this.state(timer, lang).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[unsafe(no_mangle)]
pub extern "C" fn AlternateTimingMethodComponent_state(
    this: &AlternateTimingMethodComponent,
    timer: &Timer,
    lang: Lang,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer, lang))
}
