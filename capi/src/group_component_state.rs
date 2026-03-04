//! The state object describes the information to visualize for this component.

use livesplit_core::{component::group::State as GroupComponentState, layout::ComponentState};
use std::os::raw::c_char;

/// Returns the number of components in a Group State.
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponentState_len(this: &GroupComponentState) -> usize {
    this.components.len()
}

/// Returns a string describing the type of the component at the specified
/// index within a Group State.
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponentState_component_type(
    this: &GroupComponentState,
    index: usize,
) -> *const c_char {
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
        ComponentState::Group(_) => "Group\0",
    })
    .as_ptr()
    .cast()
}

/// Returns the size override of a Group State. In horizontal mode this is the
/// height, in vertical mode it is the width. 0xFFFFFFFF means automatic
/// sizing.
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponentState_size(this: &GroupComponentState) -> u32 {
    this.size.unwrap_or(u32::MAX)
}
