//! The Previous Segment Component is a component that shows how much time was
//! saved or lost during the previous segment based on the chosen comparison.
//! Additionally, the potential time save for the previous segment can be
//! displayed. This component switches to a `Live Segment` view that shows
//! active time loss whenever the runner is losing time on the current segment.

use super::{acc, alloc, output_vec, own, own_drop, Json};
use component::OwnedComponent;
use livesplit_core::component::previous_segment::Component as PreviousSegmentComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};
use previous_segment_component_state::OwnedPreviousSegmentComponentState;

/// type
pub type OwnedPreviousSegmentComponent = *mut PreviousSegmentComponent;

/// Creates a new Previous Segment Component.
#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_new() -> OwnedPreviousSegmentComponent {
    alloc(PreviousSegmentComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_drop(this: OwnedPreviousSegmentComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_into_generic(
    this: OwnedPreviousSegmentComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_state_as_json(
    this: *const PreviousSegmentComponent,
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
pub unsafe extern "C" fn PreviousSegmentComponent_state(
    this: *const PreviousSegmentComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedPreviousSegmentComponentState {
    alloc(acc(this).state(acc(timer), acc(layout_settings)))
}
