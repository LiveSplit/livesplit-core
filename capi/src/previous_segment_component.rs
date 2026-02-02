//! The Previous Segment Component is a component that shows how much time was
//! saved or lost during the previous segment based on the chosen comparison.
//! Additionally, the potential time save for the previous segment can be
//! displayed. This component switches to a `Live Segment` view that shows
//! active time loss whenever the runner is losing time on the current segment.

use super::{Json, output_vec};
use crate::{component::OwnedComponent, key_value_component_state::OwnedKeyValueComponentState};
use livesplit_core::{
    GeneralLayoutSettings, Lang, Timer,
    component::previous_segment::Component as PreviousSegmentComponent,
};

/// type
pub type OwnedPreviousSegmentComponent = Box<PreviousSegmentComponent>;

/// Creates a new Previous Segment Component.
#[unsafe(no_mangle)]
pub extern "C" fn PreviousSegmentComponent_new() -> OwnedPreviousSegmentComponent {
    Box::new(PreviousSegmentComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn PreviousSegmentComponent_drop(this: OwnedPreviousSegmentComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn PreviousSegmentComponent_into_generic(
    this: OwnedPreviousSegmentComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn PreviousSegmentComponent_state_as_json(
    this: &PreviousSegmentComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
    lang: Lang,
) -> Json {
    output_vec(|o| {
        this.state(&timer.snapshot(), layout_settings, lang)
            .write_json(o)
            .unwrap();
    })
}

/// Calculates the component's state based on the timer and the layout
/// settings provided.
#[unsafe(no_mangle)]
pub extern "C" fn PreviousSegmentComponent_state(
    this: &PreviousSegmentComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
    lang: Lang,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(&timer.snapshot(), layout_settings, lang))
}
