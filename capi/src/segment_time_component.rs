//! The Segment Time Component is a component that shows the time for the current
//! segment in a comparison of your choosing. If no comparison is specified it
//! uses the timer's current comparison.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::component::segment_time::Component as SegmentTimeComponent;
use livesplit_core::Timer;

/// type
pub type OwnedSegmentTimeComponent = Box<SegmentTimeComponent>;

/// Creates a new Segment Time Component.
#[no_mangle]
pub extern "C" fn SegmentTimeComponent_new() -> OwnedSegmentTimeComponent {
    Box::new(SegmentTimeComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn SegmentTimeComponent_drop(this: OwnedSegmentTimeComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn SegmentTimeComponent_into_generic(
    this: OwnedSegmentTimeComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn SegmentTimeComponent_state_as_json(
    this: &SegmentTimeComponent,
    timer: &Timer,
) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub extern "C" fn SegmentTimeComponent_state(
    this: &SegmentTimeComponent,
    timer: &Timer,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer))
}
