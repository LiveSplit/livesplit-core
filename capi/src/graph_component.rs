use livesplit_core::component::graph::Component as GraphComponent;
use livesplit_core::Timer;
use super::{Json, alloc, own, own_drop, acc, output_vec};
use graph_component_state::OwnedGraphComponentState;
use component::OwnedComponent;

pub type OwnedGraphComponent = *mut GraphComponent;

#[no_mangle]
pub unsafe extern "C" fn GraphComponent_new() -> OwnedGraphComponent {
    alloc(GraphComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponent_drop(this: OwnedGraphComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponent_into_generic(this: OwnedGraphComponent) -> OwnedComponent {
    alloc(own(this).into())
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponent_state_as_json(
    this: *const GraphComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponent_state(
    this: *const GraphComponent,
    timer: *const Timer,
) -> OwnedGraphComponentState {
    alloc(acc(this).state(acc(timer)))
}
