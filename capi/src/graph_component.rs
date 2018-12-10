//! The Graph Component visualizes how far the current attempt has been ahead or
//! behind the chosen comparison throughout the whole attempt. All the
//! individual deltas are shown as points in a graph.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::graph_component_state::OwnedGraphComponentState;
use livesplit_core::component::graph::Component as GraphComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};

/// type
pub type OwnedGraphComponent = Box<GraphComponent>;

/// Creates a new Graph Component.
#[no_mangle]
pub extern "C" fn GraphComponent_new() -> OwnedGraphComponent {
    Box::new(GraphComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn GraphComponent_drop(this: OwnedGraphComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn GraphComponent_into_generic(this: OwnedGraphComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn GraphComponent_state_as_json(
    this: &GraphComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        this.state(timer, layout_settings).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer and layout settings
/// provided.
#[no_mangle]
pub extern "C" fn GraphComponent_state(
    this: &GraphComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> OwnedGraphComponentState {
    Box::new(this.state(timer, layout_settings))
}
