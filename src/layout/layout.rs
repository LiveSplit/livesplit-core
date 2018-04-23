use super::{Component, GeneralSettings, LayoutSettings, LayoutState};
use {component, Timer};

/// A Layout allows you to combine multiple components together to visualize a
/// variety of information the runner is interested in.
#[derive(Clone)]
pub struct Layout {
    /// All of the layout's components.
    pub components: Vec<Component>,
    settings: GeneralSettings,
}

impl Layout {
    /// Creates a new empty layout with no components.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            settings: GeneralSettings::default(),
        }
    }

    /// Creates a new default layout that contains a default set of components
    /// in order to provide a good default layout for runners. Which components
    /// are provided by this and how they are configured may change in the
    /// future.
    pub fn default_layout() -> Self {
        Self {
            components: vec![
                component::title::Component::new().into(),
                component::splits::Component::new().into(),
                component::timer::Component::new().into(),
                component::previous_segment::Component::new().into(),
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
    pub fn general_settings(&self) -> &GeneralSettings {
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

    /// Calculates the layout's state based on the timer provided. You can use
    /// this to visualize all of the components of a layout.
    pub fn state(&mut self, timer: &Timer) -> LayoutState {
        let settings = &self.settings;
        LayoutState {
            components: self.components
                .iter_mut()
                .map(|c| c.state(timer, settings))
                .collect(),
            background: self.settings.background,
            thin_separators_color: self.settings.thin_separators_color,
            separators_color: self.settings.separators_color,
            text_color: self.settings.text_color,
        }
    }

    /// Accesses the settings of the layout.
    pub fn settings(&self) -> LayoutSettings {
        LayoutSettings {
            components: self.components.iter().map(|c| c.settings()).collect(),
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

    /// Remounts all the components as if they were freshly initialized. Some
    /// components may only provide some information whenever it changes or when
    /// their state is first queried. Remounting returns this information again,
    /// whenever the layout's state is queried the next time.
    pub fn remount(&mut self) {
        for component in &mut self.components {
            component.remount();
        }
    }
}
