//! The Previous Segment Component is a component that shows how much time was
//! saved or lost during the previous segment based on the chosen comparison.
//! Additionally, the potential time save for the previous segment can be
//! displayed. This component switches to a `Live Segment` view that shows
//! active time loss whenever the runner is losing time on the current segment.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::component::previous_segment::Component as PreviousSegmentComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};

/// type
pub type OwnedPreviousSegmentComponent = Box<PreviousSegmentComponent>;

/// Creates a new Previous Segment Component.
#[no_mangle]
pub extern "C" fn PreviousSegmentComponent_new() -> OwnedPreviousSegmentComponent {
    Box::new(PreviousSegmentComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn PreviousSegmentComponent_drop(this: OwnedPreviousSegmentComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn PreviousSegmentComponent_into_generic(
    this: OwnedPreviousSegmentComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn PreviousSegmentComponent_state_as_json(
    this: &PreviousSegmentComponent,
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
pub extern "C" fn PreviousSegmentComponent_state(
    this: &PreviousSegmentComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer, layout_settings))
}
