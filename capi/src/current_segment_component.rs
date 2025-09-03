//! The Current Segment Component is a component that shows how much time will
//! be saved or lost during the current segment based on the chosen comparison.
//! It displays the difference between the current segment time
//! and the chosen comparison segment time.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::component::current_segment::Component as CurrentSegmentComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};

/// type
pub type OwnedCurrentSegmentComponent = Box<CurrentSegmentComponent>;

/// Creates a new Current Segment Component.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentSegmentComponent_new() -> OwnedCurrentSegmentComponent {
    Box::new(CurrentSegmentComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn CurrentSegmentComponent_drop(this: OwnedCurrentSegmentComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentSegmentComponent_into_generic(
    this: OwnedCurrentSegmentComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentSegmentComponent_state_as_json(
    this: &CurrentSegmentComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        this.state(&timer.snapshot(), layout_settings)
            .write_json(o)
            .unwrap();
    })
}

/// Calculates the component's state based on the timer and the layout
/// settings provided.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentSegmentComponent_state(
    this: &CurrentSegmentComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(&timer.snapshot(), layout_settings))
}
