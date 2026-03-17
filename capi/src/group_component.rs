//! A Component Group groups multiple components together and lays them out
//! in the opposite direction to the parent, enabling nested layout hierarchies.

use crate::component::OwnedComponent;
use livesplit_core::component::group::Component as GroupComponent;

/// type
pub type OwnedGroupComponent = Box<GroupComponent>;

/// Creates a new empty Group Component.
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponent_new() -> OwnedGroupComponent {
    Box::new(GroupComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponent_drop(this: OwnedGroupComponent) {
    drop(this);
}

/// Converts the Group Component into a generic component suitable for using
/// with a layout.
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponent_into_generic(this: OwnedGroupComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Adds a component to the end of the group.
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponent_add_component(
    this: &mut GroupComponent,
    component: OwnedComponent,
) {
    this.components.push(*component);
}

/// Returns the number of components in the group.
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponent_len(this: &GroupComponent) -> usize {
    this.components.len()
}

/// Returns the size override of the group. In horizontal mode this is the
/// height, in vertical mode it is the width. 0xFFFFFFFF means automatic sizing.
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponent_size(this: &GroupComponent) -> u32 {
    this.size.unwrap_or(u32::MAX)
}

/// Sets the size override of the group. In horizontal mode this sets the
/// height, in vertical mode it sets the width. 0xFFFFFFFF means automatic
/// sizing.
#[unsafe(no_mangle)]
pub extern "C" fn GroupComponent_set_size(this: &mut GroupComponent, size: u32) {
    this.size = if size != u32::MAX { Some(size) } else { None };
}
