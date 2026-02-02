//! Provides the PB Chance Component and relevant types for using it. The PB
//! Chance Component is a component that shows how likely it is to beat the
//! Personal Best. If there is no active attempt it shows the general chance of
//! beating the Personal Best. During an attempt it actively changes based on
//! how well the attempt is going.

use super::key_value;
use crate::{
    analysis::pb_chance,
    localization::{Lang, Text},
    platform::prelude::*,
    settings::{Color, Field, Gradient, SettingsDescription, Value},
    timing::Snapshot,
};
use core::fmt::Write;
use serde_derive::{Deserialize, Serialize};

/// The PB Chance Component is a component that shows how likely it is to beat
/// the Personal Best. If there is no active attempt it shows the general chance
/// of beating the Personal Best. During an attempt it actively changes based on
/// how well the attempt is going.
#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The background shown behind the component.
    pub background: Gradient,
    /// Specifies whether to display the name of the component and its value in
    /// two separate rows.
    pub display_two_rows: bool,
    /// The color of the label. If `None` is specified, the color is taken from
    /// the layout.
    pub label_color: Option<Color>,
    /// The color of the value. If `None` is specified, the color is taken from
    /// the layout.
    pub value_color: Option<Color>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: key_value::DEFAULT_GRADIENT,
            display_two_rows: false,
            label_color: None,
            value_color: None,
        }
    }
}

impl Component {
    /// Creates a new Possible Time Save Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Possible Time Save Component with the given settings.
    pub const fn with_settings(settings: Settings) -> Self {
        Self { settings }
    }

    /// Accesses the settings of the component.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub const fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Accesses the name of the component for the specified language.
    pub const fn name(&self, lang: Lang) -> &'static str {
        Text::ComponentPbChance.resolve(lang)
    }

    /// Updates the component's state based on the timer provided.
    pub fn update_state(&self, state: &mut key_value::State, timer: &Snapshot, lang: Lang) {
        let (chance, is_live) = pb_chance::for_timer(timer);

        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = self.settings.value_color;
        state.semantic_color = Default::default();

        state.key.clear();
        state.key.push_str(Text::ComponentPbChance.resolve(lang));

        state.value.clear();

        let chance_permille = (chance * 1000.0 + 0.5) as u32;
        let integer_part = chance_permille / 10;
        let fractional_part = chance_permille % 10;
        let decimal_separator = lang.decimal_separator();
        let _ = write!(
            state.value,
            "{integer_part}{decimal_separator}{fractional_part}%"
        );

        state.key_abbreviations.clear();
        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently = is_live;
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Snapshot, lang: Lang) -> key_value::State {
        let mut state = Default::default();
        self.update_state(&mut state, timer, lang);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values for the specified language.
    pub fn settings_description(&self, lang: Lang) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                Text::PbChanceBackground.resolve(lang).into(),
                Text::PbChanceBackgroundDescription.resolve(lang).into(),
                self.settings.background.into(),
            ),
            Field::new(
                Text::PbChanceDisplayTwoRows.resolve(lang).into(),
                Text::PbChanceDisplayTwoRowsDescription.resolve(lang).into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                Text::PbChanceLabelColor.resolve(lang).into(),
                Text::PbChanceLabelColorDescription.resolve(lang).into(),
                self.settings.label_color.into(),
            ),
            Field::new(
                Text::PbChanceValueColor.resolve(lang).into(),
                Text::PbChanceValueColorDescription.resolve(lang).into(),
                self.settings.value_color.into(),
            ),
        ])
    }

    /// Sets a setting's value by its index to the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => self.settings.display_two_rows = value.into(),
            2 => self.settings.label_color = value.into(),
            3 => self.settings.value_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
