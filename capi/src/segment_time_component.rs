//! The Segment Time Component is a component that shows the time for the current
//! segment in a comparison of your choosing. If no comparison is specified it
//! uses the timer's current comparison.

use super::{Json, output_vec};
use crate::{component::OwnedComponent, key_value_component_state::OwnedKeyValueComponentState};
use livesplit_core::{Lang, Timer, component::segment_time::Component as SegmentTimeComponent};

/// type
pub type OwnedSegmentTimeComponent = Box<SegmentTimeComponent>;

/// Creates a new Segment Time Component.
#[unsafe(no_mangle)]
pub extern "C" fn SegmentTimeComponent_new() -> OwnedSegmentTimeComponent {
    Box::new(SegmentTimeComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn SegmentTimeComponent_drop(this: OwnedSegmentTimeComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn SegmentTimeComponent_into_generic(
    this: OwnedSegmentTimeComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn SegmentTimeComponent_state_as_json(
    this: &SegmentTimeComponent,
    timer: &Timer,
    lang: Lang,
) -> Json {
    output_vec(|o| {
        this.state(timer, lang).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[unsafe(no_mangle)]
pub extern "C" fn SegmentTimeComponent_state(
    this: &SegmentTimeComponent,
    timer: &Timer,
    lang: Lang,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer, lang))
}
