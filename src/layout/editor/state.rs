use super::Editor;
use crate::{
    layout::{Component, LayoutDirection},
    localization::{Lang, Text},
    platform::prelude::*,
    settings::{ImageCache, SettingsDescription},
};
use serde_derive::{Deserialize, Serialize};

/// Represents the current state of the Layout Editor in order to visualize it
/// properly.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The name of each component in the flattened view of the layout,
    /// including components nested inside groups.
    pub components: Vec<String>,
    /// The indentation level of each component (0 = top level, 1 = inside a
    /// group, 2 = inside a nested group, etc.). Has the same length as
    /// `components`.
    pub indent_levels: Vec<u32>,
    /// Whether each component entry is a placeholder for an empty group
    /// rather than an actual component. Has the same length as `components`.
    pub is_placeholder: Vec<bool>,
    /// Describes which actions are currently available.
    pub buttons: Buttons,
    /// The flat index of the currently selected component.
    pub selected_component: u32,
    /// The layout direction at the selected component's position. This is the
    /// direction of the container that the selected component belongs to. A
    /// component added at this position would be laid out in this direction.
    /// For example, in a vertical root layout this is [`LayoutDirection::Vertical`]
    /// at the top level. Adding a group here creates a row (horizontal), adding
    /// a row's placeholder creates a column (vertical), and so on.
    pub layout_direction: LayoutDirection,
    /// A generic description of the settings available for the selected
    /// component and their current values.
    pub component_settings: SettingsDescription,
    /// A generic description of the general settings available for the layout
    /// and their current values.
    pub general_settings: SettingsDescription,
}

/// Describes which actions are currently available. Depending on how many
/// components exist and which one is selected, only some actions can be
/// executed successfully.
#[derive(Serialize, Deserialize)]
pub struct Buttons {
    /// Describes whether the currently selected component can be removed. If
    /// there's only one component in the layout, it can't be removed.
    pub can_remove: bool,
    /// Describes whether the currently selected component can be moved up. If
    /// the first sibling is selected, it can't be moved.
    pub can_move_up: bool,
    /// Describes whether the currently selected component can be moved down. If
    /// the last sibling is selected, it can't be moved.
    pub can_move_down: bool,
    /// Describes whether the currently selected component can be duplicated.
    /// Placeholders can't be duplicated.
    pub can_duplicate: bool,
}

#[cfg(feature = "std")]
impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}

impl Editor {
    /// Calculates the Layout Editor's state in order to visualize it. The
    /// [`ImageCache`] is updated with all the images that are part of the
    /// state. The images are marked as visited in the [`ImageCache`]. You still
    /// need to manually run [`ImageCache::collect`] to ensure unused images are
    /// removed from the cache.
    pub fn state(&self, image_cache: &mut ImageCache, lang: Lang) -> State {
        let root_direction = self.layout.general_settings().direction;
        let components: Vec<String> = self
            .flat
            .iter()
            .map(|entry| {
                if entry.is_placeholder {
                    Text::EmptyContainer.resolve(lang).to_owned()
                } else {
                    let parent_direction =
                        self.direction_at_path(root_direction, &entry.path[..entry.path.len() - 1]);
                    self.component_at_path(&entry.path)
                        .name(lang, parent_direction)
                        .into_owned()
                }
            })
            .collect();

        let indent_levels: Vec<u32> = self.flat.iter().map(|entry| entry.indent).collect();

        let is_placeholder: Vec<bool> =
            self.flat.iter().map(|entry| entry.is_placeholder).collect();

        let buttons = Buttons {
            can_remove: self.can_remove_component(),
            can_move_up: self.can_move_component_up(),
            can_move_down: self.can_move_component_down(),
            can_duplicate: self.can_duplicate_component(),
        };

        let component_settings = if self.flat[self.selected].is_placeholder {
            SettingsDescription::default()
        } else {
            let parent_direction = self.direction_at_path(
                root_direction,
                &self.flat[self.selected].path[..self.flat[self.selected].path.len() - 1],
            );
            self.component_at_path(&self.flat[self.selected].path)
                .settings_description(lang, parent_direction)
        };

        let layout_direction =
            self.direction_at_path(root_direction, &self.flat[self.selected].path);

        State {
            components,
            indent_levels,
            is_placeholder,
            buttons,
            selected_component: self.selected as u32,
            layout_direction,
            component_settings,
            general_settings: self
                .layout
                .general_settings()
                .settings_description(image_cache, lang),
        }
    }

    /// Returns the layout direction at a given path in the component tree.
    /// Groups flip the direction at each nesting level, while carousels
    /// preserve the parent direction.
    fn direction_at_path(&self, root: LayoutDirection, path: &[usize]) -> LayoutDirection {
        let mut direction = root;
        let mut components: &[Component] = &self.layout.components;
        for &idx in path {
            let component = &components[idx];
            match component {
                Component::Group(_) => {
                    direction = direction.opposite();
                }
                _ => break,
            }
            components = component
                .children()
                .expect("Path navigated into non-container component");
        }
        direction
    }
}
