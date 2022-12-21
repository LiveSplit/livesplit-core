//! The state object describes the information to visualize for an entire
//! layout. Use this with care, as invalid usage will result in a panic.
//!
//! Specifically, you should avoid doing the following:
//!
//! - Using out of bounds indices.
//! - Using the wrong getter function on the wrong type of component.

use crate::{output_vec, Json};
use livesplit_core::{
    component::{
        blank_space::State as BlankSpaceComponentState,
        detailed_timer::State as DetailedTimerComponentState, graph::State as GraphComponentState,
        key_value::State as KeyValueComponentState, separator::State as SeparatorComponentState,
        splits::State as SplitsComponentState, text::State as TextComponentState,
        timer::State as TimerComponentState, title::State as TitleComponentState,
    },
    layout::{ComponentState, LayoutState},
};
use std::os::raw::c_char;

/// type
pub type OwnedLayoutState = Box<LayoutState>;

/// Creates a new empty Layout State. This is useful for creating an empty
/// layout state that gets updated over time.
#[no_mangle]
pub extern "C" fn LayoutState_new() -> OwnedLayoutState {
    Default::default()
}

/// drop
#[no_mangle]
pub extern "C" fn LayoutState_drop(this: OwnedLayoutState) {
    drop(this);
}

/// Encodes the layout state as JSON. You can use this to visualize all of the
/// components of a layout.
#[no_mangle]
pub extern "C" fn LayoutState_as_json(this: &LayoutState) -> Json {
    output_vec(|o| {
        this.write_json(o).unwrap();
    })
}

/// Gets the number of Components in the Layout State.
#[no_mangle]
pub extern "C" fn LayoutState_len(this: &LayoutState) -> usize {
    this.components.len()
}

/// Returns a string describing the type of the Component at the specified
/// index.
#[no_mangle]
pub extern "C" fn LayoutState_component_type(this: &LayoutState, index: usize) -> *const c_char {
    (match this.components[index] {
        ComponentState::BlankSpace(_) => "BlankSpace\0",
        ComponentState::DetailedTimer(_) => "DetailedTimer\0",
        ComponentState::Graph(_) => "Graph\0",
        ComponentState::KeyValue(_) => "KeyValue\0",
        ComponentState::Separator(_) => "Separator\0",
        ComponentState::Splits(_) => "Splits\0",
        ComponentState::Text(_) => "Text\0",
        ComponentState::Timer(_) => "Timer\0",
        ComponentState::Title(_) => "Title\0",
    })
    .as_ptr()
    .cast()
}

/// Gets the Blank Space component state at the specified index.
#[no_mangle]
pub extern "C" fn LayoutState_component_as_blank_space(
    this: &LayoutState,
    index: usize,
) -> &BlankSpaceComponentState {
    match &this.components[index] {
        ComponentState::BlankSpace(x) => x,
        _ => panic!("wrong component state type"),
    }
}

/// Gets the Detailed Timer component state at the specified index.
#[no_mangle]
pub extern "C" fn LayoutState_component_as_detailed_timer(
    this: &LayoutState,
    index: usize,
) -> &DetailedTimerComponentState {
    match &this.components[index] {
        ComponentState::DetailedTimer(x) => x,
        _ => panic!("wrong component state type"),
    }
}

/// Gets the Graph component state at the specified index.
#[no_mangle]
pub extern "C" fn LayoutState_component_as_graph(
    this: &LayoutState,
    index: usize,
) -> &GraphComponentState {
    match &this.components[index] {
        ComponentState::Graph(x) => x,
        _ => panic!("wrong component state type"),
    }
}

/// Gets the Key Value component state at the specified index.
#[no_mangle]
pub extern "C" fn LayoutState_component_as_key_value(
    this: &LayoutState,
    index: usize,
) -> &KeyValueComponentState {
    match &this.components[index] {
        ComponentState::KeyValue(x) => x,
        _ => panic!("wrong component state type"),
    }
}

/// Gets the Separator component state at the specified index.
#[no_mangle]
pub extern "C" fn LayoutState_component_as_separator(
    this: &LayoutState,
    index: usize,
) -> &SeparatorComponentState {
    match &this.components[index] {
        ComponentState::Separator(x) => x,
        _ => panic!("wrong component state type"),
    }
}

/// Gets the Splits component state at the specified index.
#[no_mangle]
pub extern "C" fn LayoutState_component_as_splits(
    this: &LayoutState,
    index: usize,
) -> &SplitsComponentState {
    match &this.components[index] {
        ComponentState::Splits(x) => x,
        _ => panic!("wrong component state type"),
    }
}

/// Gets the Text component state at the specified index.
#[no_mangle]
pub extern "C" fn LayoutState_component_as_text(
    this: &LayoutState,
    index: usize,
) -> &TextComponentState {
    match &this.components[index] {
        ComponentState::Text(x) => x,
        _ => panic!("wrong component state type"),
    }
}

/// Gets the Timer component state at the specified index.
#[no_mangle]
pub extern "C" fn LayoutState_component_as_timer(
    this: &LayoutState,
    index: usize,
) -> &TimerComponentState {
    match &this.components[index] {
        ComponentState::Timer(x) => x,
        _ => panic!("wrong component state type"),
    }
}

/// Gets the Title component state at the specified index.
#[no_mangle]
pub extern "C" fn LayoutState_component_as_title(
    this: &LayoutState,
    index: usize,
) -> &TitleComponentState {
    match &this.components[index] {
        ComponentState::Title(x) => x,
        _ => panic!("wrong component state type"),
    }
}
