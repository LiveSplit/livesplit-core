use livesplit_core::component::previous_segment::Component as PreviousSegmentComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};
use super::{acc, alloc, output_vec, own, own_drop, Json};
use previous_segment_component_state::OwnedPreviousSegmentComponentState;
use component::OwnedComponent;

pub type OwnedPreviousSegmentComponent = *mut PreviousSegmentComponent;

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_new() -> OwnedPreviousSegmentComponent {
    alloc(PreviousSegmentComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_drop(this: OwnedPreviousSegmentComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_into_generic(
    this: OwnedPreviousSegmentComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_state_as_json(
    this: *const PreviousSegmentComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        acc(&this)
            .state(acc(&timer), acc(&layout_settings))
            .write_json(o)
            .unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_state(
    this: *const PreviousSegmentComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedPreviousSegmentComponentState {
    alloc(acc(&this).state(acc(&timer), acc(&layout_settings)))
}
