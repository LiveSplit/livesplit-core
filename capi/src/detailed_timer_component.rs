//! The Detailed Timer Component is a component that shows two timers, one for
//! the total time of the current attempt and one showing the time of just the
//! current segment. Other information, like segment times of up to two
//! comparisons, the segment icon, and the segment's name, can also be shown.

use super::{output_vec, Json};
use crate::{
    component::OwnedComponent, detailed_timer_component_state::OwnedDetailedTimerComponentState,
};
use livesplit_core::{
    component::detailed_timer::Component as DetailedTimerComponent, settings::ImageCache,
    GeneralLayoutSettings, Timer,
};

/// type
pub type OwnedDetailedTimerComponent = Box<DetailedTimerComponent>;

/// Creates a new Detailed Timer Component.
#[unsafe(no_mangle)]
pub extern "C" fn DetailedTimerComponent_new() -> OwnedDetailedTimerComponent {
    Box::new(DetailedTimerComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn DetailedTimerComponent_drop(this: OwnedDetailedTimerComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn DetailedTimerComponent_into_generic(
    this: OwnedDetailedTimerComponent,
) -> OwnedComponent {
    Box::new(Box::new(*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn DetailedTimerComponent_state_as_json(
    this: &mut DetailedTimerComponent,
    image_cache: &mut ImageCache,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        this.state(image_cache, &timer.snapshot(), layout_settings)
            .write_json(o)
            .unwrap();
    })
}

/// Calculates the component's state based on the timer and layout settings
/// provided.
#[unsafe(no_mangle)]
pub extern "C" fn DetailedTimerComponent_state(
    this: &mut DetailedTimerComponent,
    image_cache: &mut ImageCache,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> OwnedDetailedTimerComponentState {
    Box::new(this.state(image_cache, &timer.snapshot(), layout_settings))
}
