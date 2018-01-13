//! The Graph Component visualizes how far the current attempt has been ahead or
//! behind the chosen comparison throughout the whole attempt. All the
//! individual deltas are shown as points in a graph.

use livesplit_core::component::graph::Component as GraphComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};
use super::{acc, alloc, output_vec, own, own_drop, Json};
use graph_component_state::OwnedGraphComponentState;
use component::OwnedComponent;

/// type
pub type OwnedGraphComponent = *mut GraphComponent;

/// Creates a new Graph Component.
#[no_mangle]
pub unsafe extern "C" fn GraphComponent_new() -> OwnedGraphComponent {
    alloc(GraphComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn GraphComponent_drop(this: OwnedGraphComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn GraphComponent_into_generic(this: OwnedGraphComponent) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn GraphComponent_state_as_json(
    this: *const GraphComponent,
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

/// Calculates the component's state based on the timer and layout settings
/// provided.
#[no_mangle]
pub unsafe extern "C" fn GraphComponent_state(
    this: *const GraphComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedGraphComponentState {
    alloc(acc(this).state(acc(timer), acc(layout_settings)))
}
