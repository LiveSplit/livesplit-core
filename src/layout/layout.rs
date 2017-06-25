use Timer;
use super::{Component, LayoutSettings, LayoutState};

#[derive(Default, Clone)]
pub struct Layout {
    pub components: Vec<Component>,
}

impl Layout {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_settings(layout_settings: LayoutSettings) -> Self {
        Self {
            components: layout_settings
                .components
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }

    pub fn push<C: Into<Component>>(&mut self, component: C) {
        self.components.push(component.into());
    }

    pub fn state(&mut self, timer: &Timer) -> LayoutState {
        LayoutState { components: self.components.iter_mut().map(|c| c.state(timer)).collect() }
    }

    pub fn settings(&self) -> LayoutSettings {
        LayoutSettings { components: self.components.iter().map(|c| c.settings()).collect() }
    }

    pub fn scroll_up(&mut self) {
        for component in &mut self.components {
            component.scroll_up();
        }
    }

    pub fn scroll_down(&mut self) {
        for component in &mut self.components {
            component.scroll_down();
        }
    }

    pub fn remount(&mut self) {
        for component in &mut self.components {
            component.remount();
        }
    }
}
