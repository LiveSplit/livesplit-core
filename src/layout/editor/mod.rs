//! The editor module provides an editor for a [`Layout`]. The editor ensures that
//! all the different invariants of the [`Layout`] objects are upheld no matter what
//! kind of operations are being applied. It provides the current state of the
//! editor as state objects that can be visualized by any kind of User
//! Interface.

use super::{Component, Layout, LayoutState};
use crate::{
    localization::Lang,
    platform::prelude::*,
    settings::{ImageCache, Value},
    timing::Snapshot,
};
use core::result::Result as StdResult;

mod state;
#[cfg(test)]
mod tests;

pub use self::state::{Buttons as ButtonsState, State};

/// The Layout Editor allows modifying a [`Layout`] while ensuring all the
/// different invariants of the [`Layout`] objects are upheld no matter what
/// kind of operations are being applied. It provides the current state of the
/// editor as state objects that can be visualized by any kind of User
/// Interface.
///
/// All components, including those nested inside groups, are presented in a
/// single flat list. Each component has an indentation level that indicates how
/// deeply it is nested inside groups. Operations like selection, addition,
/// removal, and movement all use flat indices into this unified view.
pub struct Editor {
    layout: Layout,
    /// The flat index of the currently selected component.
    selected: usize,
    /// Cached flat list of all components. Rebuilt after every mutation.
    flat: Vec<FlatEntry>,
}

/// An entry in the flat component view, mapping a flat position to its location
/// in the component tree.
struct FlatEntry {
    /// Path of indices from the root to this component. For placeholder entries
    /// this is the path to the parent group.
    path: Vec<usize>,
    /// Indentation level (0 = top level, 1 = inside a group, etc.)
    indent: u32,
    /// When [`true`], this entry represents an "Empty" placeholder for an empty
    /// group rather than an actual component.
    is_placeholder: bool,
}

/// Describes an Error that occurred while opening the Layout Editor.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// The Layout Editor couldn't be opened because an empty layout with no
    /// components was provided.
    EmptyLayout,
}

/// The Result type for the Layout Editor.
pub type Result<T> = StdResult<T, Error>;

impl Editor {
    /// Creates a new Layout Editor that modifies the Layout provided. Creation
    /// of the Layout Editor fails when a Layout with no components is provided.
    pub fn new(layout: Layout) -> Result<Self> {
        if layout.components.is_empty() {
            return Err(Error::EmptyLayout);
        }

        let mut editor = Self {
            layout,
            selected: 0,
            flat: Vec::new(),
        };
        editor.reflatten();
        Ok(editor)
    }

    /// Rebuilds the cached flat list from the current layout, reusing the
    /// existing allocation.
    fn reflatten(&mut self) {
        self.flat.clear();
        let mut path_buf = Vec::new();
        Self::flatten_recursive(&self.layout.components, &mut path_buf, 0, &mut self.flat);
    }

    fn flatten_recursive(
        components: &[Component],
        path_prefix: &mut Vec<usize>,
        indent: u32,
        out: &mut Vec<FlatEntry>,
    ) {
        for (i, component) in components.iter().enumerate() {
            path_prefix.push(i);
            out.push(FlatEntry {
                path: path_prefix.clone(),
                indent,
                is_placeholder: false,
            });
            if let Component::Group(group) = component {
                if group.components.is_empty() {
                    // Emit a placeholder "Empty" entry for the empty group.
                    out.push(FlatEntry {
                        path: path_prefix.clone(),
                        indent: indent + 1,
                        is_placeholder: true,
                    });
                } else {
                    Self::flatten_recursive(&group.components, path_prefix, indent + 1, out);
                }
            }
            path_prefix.pop();
        }
    }

    /// Returns a reference to the component at the given tree path.
    fn component_at_path(&self, path: &[usize]) -> &Component {
        let mut components: &[Component] = &self.layout.components;
        let [parents @ .., leaf] = path else {
            unreachable!("Path must have at least one index");
        };
        for &idx in parents {
            if let Component::Group(group) = &components[idx] {
                components = &group.components;
            } else {
                unreachable!("Path navigated into non-group component");
            }
        }
        &components[*leaf]
    }

    /// Returns a mutable reference to the component at the given tree path.
    fn component_at_path_mut(&mut self, path: &[usize]) -> &mut Component {
        let mut components: &mut [Component] = &mut self.layout.components;
        let [parents @ .., leaf] = path else {
            unreachable!("Path must have at least one index");
        };
        for &idx in parents {
            if let Component::Group(group) = &mut components[idx] {
                components = &mut group.components;
            } else {
                unreachable!("Path navigated into non-group component");
            }
        }
        &mut components[*leaf]
    }

    /// Returns a reference to the component list that contains the component
    /// described by the given path (i.e. the parent container).
    fn parent_components(&self, path: &[usize]) -> &Vec<Component> {
        let mut components = &self.layout.components;
        let [parents @ .., _] = path else {
            unreachable!("Path must have at least one index");
        };
        for &idx in parents {
            if let Component::Group(group) = &components[idx] {
                components = &group.components;
            } else {
                unreachable!("Path navigated into non-group component");
            }
        }
        components
    }

    /// Returns a mutable reference to the component list that contains the
    /// component described by the given path (i.e. the parent container).
    fn parent_components_mut(&mut self, path: &[usize]) -> &mut Vec<Component> {
        let mut components = &mut self.layout.components;
        let [parents @ .., _] = path else {
            unreachable!("Path must have at least one index");
        };
        for &idx in parents {
            if let Component::Group(group) = &mut components[idx] {
                components = &mut group.components;
            } else {
                unreachable!("Path navigated into non-group component");
            }
        }
        components
    }

    /// Finds the flat index of the entry whose path matches.
    fn flat_index_for_path(flat: &[FlatEntry], path: &[usize]) -> usize {
        flat.iter().position(|e| e.path == path).unwrap()
    }

    /// Computes the flat pre-order index of a component in the
    /// [`ComponentState`] tree from a tree path. This is different from the
    /// editor's own flat index which includes placeholder entries.
    fn flat_index_of_path(components: &[crate::layout::ComponentState], path: &[usize]) -> usize {
        let mut index = 0;
        let mut components = components;
        for (depth, &child_idx) in path.iter().enumerate() {
            for c in &components[..child_idx] {
                index += Self::subtree_size_of_state(c);
            }
            if depth + 1 < path.len() {
                // Descend into the group's children; count 1 for the group
                // node itself.
                index += 1;
                if let crate::layout::ComponentState::Group(g) = &components[child_idx] {
                    components = &g.components;
                }
            }
        }
        index
    }

    /// Returns the total number of nodes in the subtree rooted at `component`
    /// (including the component itself).
    fn subtree_size_of_state(component: &crate::layout::ComponentState) -> usize {
        match component {
            crate::layout::ComponentState::Group(g) => {
                1 + g
                    .components
                    .iter()
                    .map(Self::subtree_size_of_state)
                    .sum::<usize>()
            }
            _ => 1,
        }
    }

    /// Returns the flat index one past the last item in the subtree rooted at
    /// `flat_index`.
    fn subtree_end(flat_index: usize, flat: &[FlatEntry]) -> usize {
        let base_indent = flat[flat_index].indent;
        let mut end = flat_index + 1;
        while end < flat.len() && flat[end].indent > base_indent {
            end += 1;
        }
        end
    }

    /// Updates `self.selected` to the flat index matching `path`, after
    /// rebuilding the cached flat list.
    fn select_path(&mut self, path: &[usize]) {
        self.reflatten();
        self.selected = Self::flat_index_for_path(&self.flat, path);
    }

    /// Returns the currently selected entry.
    fn selected_entry(&self) -> &FlatEntry {
        &self.flat[self.selected]
    }

    /// Closes the Layout Editor and gives back access to the modified Layout.
    /// In case you want to implement a Cancel Button, just drop the Layout
    /// object you get here.
    pub fn close(self) -> Layout {
        self.layout
    }

    /// Calculates the layout's state based on the timer provided. You can use
    /// this to visualize all of the components of a layout, while it is still
    /// being edited by the Layout Editor. The [`ImageCache`] is updated with
    /// all the images that are part of the state. The images are marked as
    /// visited in the [`ImageCache`]. You still need to manually run
    /// [`ImageCache::collect`] to ensure unused images are removed from the
    /// cache.
    pub fn layout_state(
        &mut self,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        lang: Lang,
    ) -> LayoutState {
        let mut state = self.layout.state(image_cache, timer, lang);
        state.selected_component = Some(Self::flat_index_of_path(
            &state.components,
            &self.selected_entry().path,
        ));
        state
    }

    /// Updates the layout's state based on the timer provided. You can use this
    /// to visualize all of the components of a layout, while it is still being
    /// edited by the Layout Editor. The [`ImageCache`] is updated with all the
    /// images that are part of the state. The images are marked as visited in
    /// the [`ImageCache`]. You still need to manually run
    /// [`ImageCache::collect`] to ensure unused images are removed from the
    /// cache.
    pub fn update_layout_state(
        &mut self,
        state: &mut LayoutState,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        lang: Lang,
    ) {
        self.layout.update_state(state, image_cache, timer, lang);
        state.selected_component = Some(Self::flat_index_of_path(
            &state.components,
            &self.selected_entry().path,
        ));
    }

    /// Selects the component at the given flat index in order to modify its
    /// settings. Only a single component is selected at any given time. You may
    /// not provide an invalid index.
    pub const fn select(&mut self, index: usize) {
        if index < self.flat.len() {
            self.selected = index;
        }
    }

    /// Adds the component provided to the layout. If the currently selected
    /// entry is an empty-group placeholder, the new component is inserted as
    /// the first child of that group. Otherwise it is added as a sibling right
    /// after the currently selected component (and all of its children in the
    /// case of a group). The newly added component becomes the selected
    /// component.
    pub fn add_component<C: Into<Component>>(&mut self, component: C) {
        let entry = self.selected_entry();

        if entry.is_placeholder {
            // Placeholder or group is selected – add as first child.
            let group_path = entry.path.clone();
            if let Component::Group(group) = self.component_at_path_mut(&group_path) {
                group.components.insert(0, component.into());
            }
            let mut new_path = group_path;
            new_path.push(0);
            self.select_path(&new_path);
        } else {
            // Regular component – add as sibling right after it.
            let path = entry.path.clone();
            let sibling_index = *path.last().unwrap();
            let subtree_end = Self::subtree_end(self.selected, &self.flat);

            let components = self.parent_components_mut(&path);
            components.insert(sibling_index + 1, component.into());

            self.reflatten();
            self.selected = subtree_end;
        }

        // If we just added a group, automatically select its empty placeholder
        // so the user can immediately start adding components inside it.
        if !self.flat[self.selected].is_placeholder
            && matches!(
                self.component_at_path(&self.flat[self.selected].path),
                Component::Group(_)
            )
        {
            self.selected += 1;
        }
    }

    /// Checks if the currently selected component can be removed. Placeholders
    /// can't be removed. At the top level, at least one component must remain.
    pub fn can_remove_component(&self) -> bool {
        let entry = self.selected_entry();
        if entry.is_placeholder {
            return false;
        }
        if entry.path.len() == 1 {
            self.layout.components.len() > 1
        } else {
            true
        }
    }

    /// Removes the currently selected component (and all its children if it is
    /// a group), unless there's only one component at the top level. The next
    /// sibling becomes the selected component. If there's none, the previous
    /// sibling becomes the selected component instead. If the parent group
    /// becomes empty, the group itself becomes selected.
    pub fn remove_component(&mut self) {
        if !self.can_remove_component() {
            return;
        }

        let path = self.selected_entry().path.clone();
        let sibling_index = *path.last().unwrap();

        {
            let components = self.parent_components_mut(&path);
            components.remove(sibling_index);
        }

        let remaining = self.parent_components(&path).len();
        let new_path = if remaining == 0 {
            path[..path.len() - 1].to_vec()
        } else {
            let new_sibling = sibling_index.min(remaining - 1);
            let mut p = path[..path.len() - 1].to_vec();
            p.push(new_sibling);
            p
        };
        self.select_path(&new_path);
    }

    /// Checks if the currently selected component can be moved up. Placeholders
    /// can't be moved. If the selected component is the first sibling in its
    /// parent, it can't be moved up.
    pub fn can_move_component_up(&self) -> bool {
        let entry = self.selected_entry();
        !entry.is_placeholder && *entry.path.last().unwrap() > 0
    }

    /// Moves the selected component up within its parent, unless the first
    /// sibling is selected.
    pub fn move_component_up(&mut self) {
        if !self.can_move_component_up() {
            return;
        }
        let path = self.selected_entry().path.clone();
        let sibling_index = *path.last().unwrap();

        let components = self.parent_components_mut(&path);
        components.swap(sibling_index, sibling_index - 1);

        let mut new_path = path[..path.len() - 1].to_vec();
        new_path.push(sibling_index - 1);
        self.select_path(&new_path);
    }

    /// Checks if the currently selected component can be moved down. Placeholders
    /// can't be moved. If the selected component is the last sibling in its
    /// parent, it can't be moved down.
    pub fn can_move_component_down(&self) -> bool {
        let entry = self.selected_entry();
        if entry.is_placeholder {
            return false;
        }
        let sibling_index = *entry.path.last().unwrap();
        let parent_len = self.parent_components(&entry.path).len();
        sibling_index + 1 < parent_len
    }

    /// Moves the selected component down within its parent, unless the last
    /// sibling is selected.
    pub fn move_component_down(&mut self) {
        if !self.can_move_component_down() {
            return;
        }
        let path = self.selected_entry().path.clone();
        let sibling_index = *path.last().unwrap();

        let components = self.parent_components_mut(&path);
        components.swap(sibling_index, sibling_index + 1);

        let mut new_path = path[..path.len() - 1].to_vec();
        new_path.push(sibling_index + 1);
        self.select_path(&new_path);
    }

    /// Moves the selected component to the given flat index position. The
    /// component is removed from its current position and inserted at the
    /// target position, potentially moving across group boundaries. Moving
    /// into the component's own subtree is not allowed. If the target is an
    /// empty-group placeholder the component is inserted as the first child
    /// of that group. Placeholders themselves cannot be moved.
    pub fn move_component(&mut self, dst_flat_index: usize) {
        if dst_flat_index >= self.flat.len() || dst_flat_index == self.selected {
            return;
        }

        if self.selected_entry().is_placeholder {
            return;
        }

        let subtree_end = Self::subtree_end(self.selected, &self.flat);
        if dst_flat_index > self.selected && dst_flat_index < subtree_end {
            return; // Can't move into own subtree
        }

        let src_path = self.flat[self.selected].path.clone();
        let dst_path = self.flat[dst_flat_index].path.clone();
        let dst_is_placeholder = self.flat[dst_flat_index].is_placeholder;

        // Remove source component from its parent.
        let src_sibling_idx = *src_path.last().unwrap();
        let component = self
            .parent_components_mut(&src_path)
            .remove(src_sibling_idx);

        // Adjust the destination path to account for the removal.
        let adjusted = Self::adjust_path_after_removal(&dst_path, &src_path);

        if dst_is_placeholder {
            // The placeholder's path points to the empty group – insert as first child.
            if let Component::Group(group) = self.component_at_path_mut(&adjusted) {
                group.components.insert(0, component);
            }
            let mut new_path = adjusted;
            new_path.push(0);
            self.select_path(&new_path);
        } else {
            let parent_depth = adjusted.len() - 1;
            let dst_sibling_idx = adjusted[parent_depth];

            let insert_idx = if dst_flat_index > self.selected {
                // Moving down: insert after the destination component.
                dst_sibling_idx + 1
            } else {
                // Moving up: insert at the destination component's position.
                dst_sibling_idx
            };

            let parent = self.parent_components_mut(&adjusted);
            parent.insert(insert_idx, component);

            let mut new_path = adjusted[..parent_depth].to_vec();
            new_path.push(insert_idx);
            self.select_path(&new_path);
        }
    }

    /// Adjusts `dst_path` to account for the removal of the component at
    /// `removed_path`. If they share the same parent and the destination was
    /// after the removed entry, the sibling index is decremented.
    fn adjust_path_after_removal(dst_path: &[usize], removed_path: &[usize]) -> Vec<usize> {
        let mut result = dst_path.to_vec();
        let parent_depth = removed_path.len() - 1;
        let removed_idx = removed_path[parent_depth];

        if result.len() > parent_depth
            && result[..parent_depth] == removed_path[..parent_depth]
            && result[parent_depth] > removed_idx
        {
            result[parent_depth] -= 1;
        }

        result
    }

    /// Checks if the currently selected component can be duplicated.
    /// Placeholders can't be duplicated.
    pub fn can_duplicate_component(&self) -> bool {
        !self.selected_entry().is_placeholder
    }

    /// Duplicates the currently selected component. The copy gets placed right
    /// after the selected component and becomes the newly selected component.
    /// Placeholders cannot be duplicated.
    pub fn duplicate_component(&mut self) {
        if !self.can_duplicate_component() {
            return;
        }
        let path = self.selected_entry().path.clone();
        let sibling_index = *path.last().unwrap();
        let subtree_end = Self::subtree_end(self.selected, &self.flat);

        let components = self.parent_components_mut(&path);
        let component = components[sibling_index].clone();
        components.insert(sibling_index + 1, component);

        self.reflatten();
        self.selected = subtree_end;
    }

    /// Sets a setting's value of the selected component by its setting index
    /// to the given value. Has no effect when a placeholder is selected.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_component_settings_value(&mut self, index: usize, value: Value) {
        if self.selected_entry().is_placeholder {
            return;
        }
        let path = self.selected_entry().path.clone();
        self.component_at_path_mut(&path).set_value(index, value);
    }

    /// Sets a setting's value of the general settings by its setting index to
    /// the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_general_settings_value(
        &mut self,
        index: usize,
        value: Value,
        image_cache: &ImageCache,
    ) {
        self.layout
            .general_settings_mut()
            .set_value(index, value, image_cache);
    }
}
