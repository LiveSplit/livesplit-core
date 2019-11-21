use super::Editor;
use crate::platform::prelude::*;
use crate::settings::SettingsDescription;
use serde::{Deserialize, Serialize};

/// Represents the current state of the Layout Editor in order to visualize it
/// properly.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The name of all the components in the layout.
    pub components: Vec<String>,
    /// Describes which actions are currently available.
    pub buttons: Buttons,
    /// The index of the currently selected component.
    pub selected_component: u32,
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
    /// the first component is selected, it can't be moved.
    pub can_move_up: bool,
    /// Describes whether the currently selected component can be moved down. If
    /// the last component is selected, it can't be moved.
    pub can_move_down: bool,
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
    /// Calculates the Layout Editor's state in order to visualize it.
    pub fn state(&self) -> State {
        let components = self
            .layout
            .components
            .iter()
            .map(|c| c.name().into_owned())
            .collect();

        let buttons = Buttons {
            can_remove: self.can_remove_component(),
            can_move_up: self.can_move_component_up(),
            can_move_down: self.can_move_component_down(),
        };

        State {
            components,
            buttons,
            selected_component: self.selected_component as u32,
            component_settings: self.layout.components[self.selected_component]
                .settings_description(),
            general_settings: self.layout.general_settings().settings_description(),
        }
    }
}
