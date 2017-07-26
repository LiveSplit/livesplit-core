use serde_json::{to_writer, Result as JsonResult};
use std::io::Write;
use super::Editor;
use settings::SettingsDescription;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub components: Vec<String>,
    pub buttons: Buttons,
    pub selected_component: u32,
    pub component_settings: SettingsDescription,
    pub general_settings: SettingsDescription,
}

#[derive(Serialize, Deserialize)]
pub struct Buttons {
    pub can_remove: bool,
    pub can_move_up: bool,
    pub can_move_down: bool,
}

impl State {
    pub fn write_json<W>(&self, writer: W) -> JsonResult<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Editor {
    pub fn state(&self) -> State {
        let components = self.layout
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
