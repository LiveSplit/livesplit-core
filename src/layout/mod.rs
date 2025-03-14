//! The layout module provides everything necessary for working with a
//! [`Layout`]. A [`Layout`] allows you to combine multiple components together
//! to visualize a variety of information the runner is interested in.

mod component;
mod component_settings;
mod component_state;
pub mod editor;
mod general_settings;
mod layout_direction;
mod layout_settings;
mod layout_state;
pub mod parser;

pub use self::{
    component::Component, component_settings::ComponentSettings, component_state::ComponentState,
    editor::Editor, general_settings::GeneralSettings, layout_direction::LayoutDirection,
    layout_settings::LayoutSettings, layout_state::LayoutState,
};

use crate::{
    component::{previous_segment, splits, timer, title},
    platform::prelude::*,
    settings::ImageCache,
    timing::Snapshot,
};

/// A Layout allows you to combine multiple components together to visualize a
/// variety of information the runner is interested in.
#[derive(Clone, Default)]
pub struct Layout {
    /// All of the layout's components.
    pub components: Vec<Component>,
    settings: GeneralSettings,
}

impl Layout {
    /// Creates a new empty layout with no components.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new default layout that contains a default set of components
    /// in order to provide a good default layout for runners. Which components
    /// are provided by this and how they are configured may change in the
    /// future.
    pub fn default_layout() -> Self {
        Self {
            components: vec![
                title::Component::new().into(),
                splits::Component::new().into(),
                timer::Component::new().into(),
                previous_segment::Component::new().into(),
            ],
            settings: GeneralSettings::default(),
        }
    }

    /// Creates a new layout from the layout settings of the whole layout.
    pub fn from_settings(layout_settings: LayoutSettings) -> Self {
        Self {
            components: layout_settings
                .components
                .into_iter()
                .map(Into::into)
                .collect(),
            settings: layout_settings.general,
        }
    }

    /// Accesses the general settings of the layout that apply to all
    /// components.
    pub const fn general_settings(&self) -> &GeneralSettings {
        &self.settings
    }

    /// Grants mutable access to the general settings of the layout that apply
    /// to all components.
    pub fn general_settings_mut(&mut self) -> &mut GeneralSettings {
        &mut self.settings
    }

    /// Adds a new component to the end of the layout.
    pub fn push<C: Into<Component>>(&mut self, component: C) {
        self.components.push(component.into());
    }

    /// Updates the layout's state based on the timer provided. You can use this
    /// to visualize all of the components of a layout. The [`ImageCache`] is
    /// updated with all the images that are part of the state. The images are
    /// marked as visited in the [`ImageCache`]. You still need to manually run
    /// [`ImageCache::collect`] to ensure unused images are removed from the
    /// cache.
    pub fn update_state(
        &mut self,
        state: &mut LayoutState,
        image_cache: &mut ImageCache,
        timer: &Snapshot<'_>,
    ) {
        let settings = &self.settings;

        state.components.truncate(self.components.len());
        let mut components = self.components.iter_mut();
        // First update all the states that we have.
        for (state, component) in state.components.iter_mut().zip(components.by_ref()) {
            component.update_state(state, image_cache, timer, settings);
        }
        // Then add states for all the components that don't have states yet.
        state
            .components
            .extend(components.map(|c| c.state(image_cache, timer, settings)));

        state.timer_font.clone_from(&settings.timer_font);
        state.times_font.clone_from(&settings.times_font);
        state.text_font.clone_from(&settings.text_font);

        state.background = settings.background.cache(image_cache);
        state.thin_separators_color = settings.thin_separators_color;
        state.separators_color = settings.separators_color;
        state.text_color = settings.text_color;
        state.direction = settings.direction;
        state.mouse_pass_through_while_running = settings.mouse_pass_through_while_running;
        state.drop_shadow = settings.drop_shadow;
    }

    /// Calculates the layout's state based on the timer provided. You can use
    /// this to visualize all of the components of a layout. The [`ImageCache`]
    /// is updated with all the images that are part of the state. The images
    /// are marked as visited in the [`ImageCache`]. You still need to manually
    /// run [`ImageCache::collect`] to ensure unused images are removed from the
    /// cache.
    pub fn state(&mut self, image_cache: &mut ImageCache, timer: &Snapshot<'_>) -> LayoutState {
        let mut state = Default::default();
        self.update_state(&mut state, image_cache, timer);
        state
    }

    /// Accesses the settings of the layout.
    pub fn settings(&self) -> LayoutSettings {
        LayoutSettings {
            components: self.components.iter().map(Component::settings).collect(),
            general: self.settings.clone(),
        }
    }

    /// Scrolls up all the components in the layout that can be scrolled up.
    pub fn scroll_up(&mut self) {
        for component in &mut self.components {
            component.scroll_up();
        }
    }

    /// Scrolls down all the components in the layout that can be scrolled down.
    pub fn scroll_down(&mut self) {
        for component in &mut self.components {
            component.scroll_down();
        }
    }
}
