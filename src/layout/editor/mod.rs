//! The editor module provides an editor for Layouts. The editor ensures that
//! all the different invariants of the Layout objects are upheld no matter what
//! kind of operations are being applied. It provides the current state of the
//! editor as state objects that can be visualized by any kind of User
//! Interface.

use super::{Component, Layout, LayoutState};
use crate::settings::Value;
use crate::Timer;
use core::result::Result as StdResult;

mod state;

pub use self::state::{Buttons as ButtonsState, State};

/// The Layout Editor allows modifying Layouts while ensuring all the different
/// invariants of the Layout objects are upheld no matter what kind of
/// operations are being applied. It provides the current state of the editor as
/// state objects that can be visualized by any kind of User Interface.
pub struct Editor {
    layout: Layout,
    selected_component: usize,
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
    pub fn new(mut layout: Layout) -> Result<Self> {
        if layout.components.is_empty() {
            return Err(Error::EmptyLayout);
        }

        layout.remount();

        Ok(Self {
            layout,
            selected_component: 0,
        })
    }

    /// Closes the Layout Editor and gives back access to the modified Layout.
    /// In case you want to implement a Cancel Button, just drop the Layout
    /// object you get here.
    pub fn close(self) -> Layout {
        self.layout
    }

    /// Calculates the layout's state based on the timer provided. You can use
    /// this to visualize all of the components of a layout, while it is still
    /// being edited by the Layout Editor.
    pub fn layout_state(&mut self, timer: &Timer) -> LayoutState {
        self.layout.state(timer)
    }

    /// Selects the component with the given index in order to modify its
    /// settings. Only a single component is selected at any given time. You may
    /// not provide an invalid index.
    pub fn select(&mut self, index: usize) {
        if index < self.layout.components.len() {
            self.selected_component = index;
        }
    }

    /// Adds the component provided to the end of the layout. The newly added
    /// component becomes the selected component.
    pub fn add_component<C: Into<Component>>(&mut self, component: C) {
        self.selected_component = self.layout.components.len();
        self.layout.push(component);
    }

    /// Checks if the currently selected component can be removed. If there's
    /// only one component in the layout, it can't be removed.
    pub fn can_remove_component(&self) -> bool {
        // We need to ensure there's always at least one component.
        self.layout.components.len() > 1
    }

    /// Removes the currently selected component, unless there's only one
    /// component in the layout. The next component becomes the selected
    /// component. If there's none, the previous component becomes the selected
    /// component instead.
    pub fn remove_component(&mut self) {
        if self.can_remove_component() {
            self.layout.components.remove(self.selected_component);
            if self.selected_component >= self.layout.components.len() {
                self.selected_component = self.layout.components.len() - 1;
            }
            self.layout.remount();
        }
    }

    /// Checks if the currently selected component can be moved up. If the first
    /// component is selected, it can't be moved up.
    pub fn can_move_component_up(&self) -> bool {
        self.selected_component > 0
    }

    /// Moves the selected component up, unless the first component is selected.
    pub fn move_component_up(&mut self) {
        if self.can_move_component_up() {
            self.layout
                .components
                .swap(self.selected_component, self.selected_component - 1);
            self.selected_component -= 1;
            self.layout.remount();
        }
    }

    /// Checks if the currently selected component can be moved down. If the
    /// last component is selected, it can't be moved down.
    pub fn can_move_component_down(&self) -> bool {
        self.selected_component < self.layout.components.len() - 1
    }

    /// Moves the selected component down, unless the last component is
    /// selected.
    pub fn move_component_down(&mut self) {
        if self.can_move_component_down() {
            self.layout
                .components
                .swap(self.selected_component, self.selected_component + 1);
            self.selected_component += 1;
            self.layout.remount();
        }
    }

    /// Moves the selected component to the index provided. You may not provide
    /// an invalid index.
    pub fn move_component(&mut self, dst_index: usize) {
        if dst_index < self.layout.components.len() {
            while self.selected_component > dst_index {
                self.move_component_up();
            }
            while self.selected_component < dst_index {
                self.move_component_down();
            }
        }
    }

    /// Duplicates the currently selected component. The copy gets placed right
    /// after the selected component and becomes the newly selected component.
    pub fn duplicate_component(&mut self) {
        let index = self.selected_component;
        let new_index = index + 1;

        let component = self.layout.components[index].clone();
        self.layout.components.insert(new_index, component);

        self.selected_component = new_index;
        self.layout.remount();
    }

    /// Sets a setting's value of the selected component by its setting index
    /// to the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_component_settings_value(&mut self, index: usize, value: Value) {
        self.layout.components[self.selected_component].set_value(index, value);
    }

    /// Sets a setting's value of the general settings by its setting index to
    /// the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_general_settings_value(&mut self, index: usize, value: Value) {
        self.layout.general_settings_mut().set_value(index, value);
    }
}
