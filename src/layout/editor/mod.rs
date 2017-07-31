use super::{Layout, LayoutState, Component};
use settings::Value;
use std::result::Result as StdResult;
use Timer;

mod state;

pub use self::state::{State, Buttons as ButtonsState};

pub struct Editor {
    layout: Layout,
    selected_component: usize,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        EmptyLayout
    }
}

pub type Result<T> = StdResult<T, Error>;

impl Editor {
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

    pub fn close(self) -> Layout {
        self.layout
    }

    pub fn layout_state(&mut self, timer: &Timer) -> LayoutState {
        self.layout.state(timer)
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
            self.layout.remount();
        }
    }

    pub fn can_move_component_up(&self) -> bool {
        self.selected_component > 0
    }

    pub fn move_component_up(&mut self) {
        if self.can_move_component_up() {
            self.layout
                .components
                .swap(self.selected_component, self.selected_component - 1);
            self.selected_component -= 1;
            self.layout.remount();
        }
    }

    pub fn can_move_component_down(&self) -> bool {
        self.selected_component < self.layout.components.len() - 1
    }

    pub fn move_component_down(&mut self) {
        if self.can_move_component_down() {
            self.layout
                .components
                .swap(self.selected_component, self.selected_component + 1);
            self.selected_component += 1;
            self.layout.remount();
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

    pub fn set_general_settings_value(&mut self, index: usize, value: Value) {
        self.layout.general_settings_mut().set_value(index, value);
    }
}
