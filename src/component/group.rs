//! Provides the Component Group and relevant types for using it. A Component
//! Group allows nesting components in a layout direction opposite to its
//! parent, enabling horizontal groups inside vertical layouts and vice versa.

use core::hash::{Hash, Hasher};

use crate::{
    layout::{self, LayoutDirection},
    localization::{Lang, Text},
    platform::prelude::*,
    settings::{Field, ImageCache, SettingsDescription, Value},
    timing::Snapshot,
};
use serde_derive::{Deserialize, Serialize};

/// A group of components that are laid out together in the opposite direction
/// to their parent, enabling nested layout hierarchies. For example, a
/// vertical layout can contain a horizontal group of components.
#[derive(Clone)]
pub struct Component {
    /// The components contained in this group.
    pub components: Vec<layout::Component>,
    /// An optional size override for the group. In horizontal mode this sets
    /// the height, in vertical mode it sets the width. [`None`] means the size
    /// is determined automatically from the children.
    pub size: Option<u32>,
}

impl Default for Component {
    fn default() -> Self {
        Self::new()
    }
}

/// The state object for a component group, containing the states of its
/// children.
#[derive(Default, Serialize, Deserialize)]
pub struct State {
    /// An optional size override. In horizontal mode this sets the height, in
    /// vertical mode it sets the width. [`None`] means automatic sizing.
    pub size: Option<u32>,
    /// The state objects for the components in this group.
    pub components: Vec<layout::ComponentState>,
}

impl State {
    pub(crate) fn has_same_content(&self, other: &Self) -> bool {
        self.components.len() == other.components.len()
            && self
                .components
                .iter()
                .zip(other.components.iter())
                .all(|(a, b)| a.has_same_content(b))
    }

    pub(crate) fn content_fingerprint(&self, state: &mut impl Hasher) {
        self.components.len().hash(state);
        for component in &self.components {
            component.content_fingerprint().hash(state);
        }
    }
}

/// The serializable settings for a component group.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// An optional size override. In horizontal mode this sets the height, in
    /// vertical mode it sets the width. [`None`] means automatic sizing.
    pub size: Option<u32>,
    /// The settings for each component in the group.
    pub components: Vec<layout::ComponentSettings>,
}

impl Component {
    /// Creates a new empty component group.
    pub const fn new() -> Self {
        Self {
            components: Vec::new(),
            size: None,
        }
    }

    /// Creates a new Component Group with the given settings.
    pub fn with_settings(settings: Settings) -> Self {
        Self {
            size: settings.size,
            components: settings.components.into_iter().map(Into::into).collect(),
        }
    }

    /// Accesses the name of the component for the specified language and
    /// direction. The direction is the direction the group lays out its
    /// children in. A horizontal group is called a "Row" and a vertical group
    /// is called a "Column".
    pub const fn name(&self, lang: Lang, direction: LayoutDirection) -> &'static str {
        match direction {
            LayoutDirection::Horizontal => Text::Row.resolve(lang),
            LayoutDirection::Vertical => Text::Column.resolve(lang),
        }
    }

    /// Calculates the group's state.
    pub fn state(
        &mut self,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        layout_settings: &layout::GeneralSettings,
        lang: Lang,
    ) -> State {
        State {
            size: self.size,
            components: self
                .components
                .iter_mut()
                .map(|c| c.state(image_cache, timer, layout_settings, lang))
                .collect(),
        }
    }

    /// Updates the group's state in place.
    pub fn update_state(
        &mut self,
        state: &mut State,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        layout_settings: &layout::GeneralSettings,
        lang: Lang,
    ) {
        state.size = self.size;
        state.components.truncate(self.components.len());
        let mut components = self.components.iter_mut();
        for (state, component) in state.components.iter_mut().zip(components.by_ref()) {
            component.update_state(state, image_cache, timer, layout_settings, lang);
        }
        state
            .components
            .extend(components.map(|c| c.state(image_cache, timer, layout_settings, lang)));
    }

    /// Returns the settings for this group.
    pub fn settings(&self) -> Settings {
        Settings {
            size: self.size,
            components: self
                .components
                .iter()
                .map(layout::Component::settings)
                .collect(),
        }
    }

    /// Returns a settings description for this group.
    pub fn settings_description(
        &self,
        lang: Lang,
        direction: LayoutDirection,
    ) -> SettingsDescription {
        let (text, description) = match direction {
            LayoutDirection::Vertical => (Text::GroupFixedWidth, Text::GroupFixedWidthDescription),
            LayoutDirection::Horizontal => {
                (Text::GroupFixedHeight, Text::GroupFixedHeightDescription)
            }
        };
        SettingsDescription::with_fields(vec![Field::new(
            text.resolve(lang).into(),
            description.resolve(lang).into(),
            self.size.map(|s| s as u64).into(),
        )])
    }

    /// Sets a setting value by index.
    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => {
                self.size =
                    Option::<u64>::from(value).map(|v| u32::try_from(v).unwrap_or(u32::MAX));
            }
            _ => panic!("Unsupported setting index"),
        }
    }

    /// Tells each component in the group to scroll up.
    pub fn scroll_up(&mut self) {
        for c in &mut self.components {
            c.scroll_up();
        }
    }

    /// Tells each component in the group to scroll down.
    pub fn scroll_down(&mut self) {
        for c in &mut self.components {
            c.scroll_down();
        }
    }
}
