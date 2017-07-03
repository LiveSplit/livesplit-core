use super::{Layout, Component};

pub mod settings_description;
mod state;

pub use self::state::{State, Buttons as ButtonsState};
use self::settings_description::Value;

pub struct LayoutEditor {
    layout: Layout,
    selected_component: usize,
}

impl LayoutEditor {
    pub fn new(layout: Layout) -> Self {
        let len = layout.components.len();
        assert!(len > 0);

        LayoutEditor {
            layout,
            selected_component: 0,
        }
    }

    pub fn close(self) -> Layout {
        self.layout
    }

    pub fn select(&mut self, index: usize) {
        if index < self.layout.components.len() {
            self.selected_component = index;
        }
    }

    pub fn add_component<C: Into<Component>>(&mut self, component: C) {
        self.selected_component = self.layout.components.len();
        self.layout.push(component);
    }

    pub fn can_remove_component(&self) -> bool {
        // We need to ensure there's always at least one component.
        self.layout.components.len() > 1
    }

    pub fn remove_component(&mut self) {
        if self.can_remove_component() {
            self.layout.components.remove(self.selected_component);
            if self.selected_component >= self.layout.components.len() {
                self.selected_component = self.layout.components.len() - 1;
            }
        }
    }

    pub fn can_move_component_up(&self) -> bool {
        self.selected_component > 0
    }

    pub fn move_component_up(&mut self) {
        if self.can_move_component_up() {
            self.layout.components.swap(
                self.selected_component,
                self.selected_component - 1,
            );
            self.selected_component -= 1;
        }
    }

    pub fn can_move_component_down(&self) -> bool {
        self.selected_component < self.layout.components.len() - 1
    }

    pub fn move_component_down(&mut self) {
        if self.can_move_component_down() {
            self.layout.components.swap(
                self.selected_component,
                self.selected_component + 1,
            );
            self.selected_component += 1;
        }
    }

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

    pub fn set_component_settings_value(&mut self, index: usize, value: Value) {
        self.layout.components[self.selected_component].set_value(index, value);
    }
}
