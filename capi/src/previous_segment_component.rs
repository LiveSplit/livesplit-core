use livesplit_core::component::previous_segment::Component as PreviousSegmentComponent;
use livesplit_core::Timer;
use super::{Json, alloc, own_drop, acc, output_vec};
use previous_segment_component_state::OwnedPreviousSegmentComponentState;

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
pub unsafe extern "C" fn PreviousSegmentComponent_state_as_json(this: *const PreviousSegmentComponent,
timer: *const Timer) -> Json{
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_state(this: *const PreviousSegmentComponent,
                                                        timer: *const Timer)
                                                        -> OwnedPreviousSegmentComponentState {
    alloc(acc(this).state(acc(timer)))
}
