//! Provides the Possible Time Save Component and relevant types for using it.
//! The Possible Time Save Component is a component that shows how much time the
//! chosen comparison could've saved for the current segment, based on the Best
//! Segments. This component also allows showing the Total Possible Time Save
//! for the remainder of the current attempt.

use super::key_value;
use crate::{
    analysis::possible_time_save,
    comparison,
    platform::prelude::*,
    settings::{Color, Field, Gradient, SettingsDescription, Value},
    timing::{
        formatter::{Accuracy, SegmentTime, TimeFormatter},
        Snapshot,
    },
    TimerPhase,
};
use alloc::borrow::Cow;
use core::fmt::Write as FmtWrite;
use serde_derive::{Deserialize, Serialize};

/// The Possible Time Save Component is a component that shows how much time the
/// chosen comparison could've saved for the current segment, based on the Best
/// Segments. This component also allows showing the Total Possible Time Save
/// for the remainder of the current attempt.
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
    /// The comparison chosen. Uses the Timer's current comparison if set to
    /// `None`.
    pub comparison_override: Option<String>,
    /// Specifies whether to display the name of the component and its value in
    /// two separate rows.
    pub display_two_rows: bool,
    /// Activates the Total Possible Time Save mode, where the remaining time
    /// save for the current attempt is shown, instead of the time save for the
    /// current segment.
    pub total_possible_time_save: bool,
    /// The color of the label. If `None` is specified, the color is taken from
    /// the layout.
    pub label_color: Option<Color>,
    /// The color of the value. If `None` is specified, the color is taken from
    /// the layout.
    pub value_color: Option<Color>,
    /// The accuracy of the time shown.
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: key_value::DEFAULT_GRADIENT,
            comparison_override: None,
            display_two_rows: false,
            total_possible_time_save: false,
            label_color: None,
            value_color: None,
            accuracy: Accuracy::Hundredths,
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
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> Cow<'static, str> {
        self.text(
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_ref),
        )
    }

    fn text(&self, comparison: Option<&str>) -> Cow<'static, str> {
        let text = if self.settings.total_possible_time_save {
            "Total Possible Time Save"
        } else {
            "Possible Time Save"
        };
        let mut text = Cow::from(text);
        if let Some(comparison) = comparison {
            write!(text.to_mut(), " ({})", comparison::shorten(comparison)).unwrap();
        }
        text
    }

    /// Updates the component's state based on the timer provided.
    pub fn update_state(&self, state: &mut key_value::State, timer: &Snapshot<'_>) {
        let segment_index = timer.current_split_index();
        let current_phase = timer.current_phase();
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let text = self.text(comparison);
        let comparison = comparison::or_current(comparison, timer);

        let (time, updates_frequently) = if self.settings.total_possible_time_save {
            let (time, updates_frequently) =
                possible_time_save::calculate_total(timer, segment_index.unwrap_or(0), comparison);
            (Some(time), updates_frequently)
        } else if current_phase == TimerPhase::Running || current_phase == TimerPhase::Paused {
            possible_time_save::calculate(timer, segment_index.unwrap(), comparison, false)
        } else {
            (None, false)
        };

        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = self.settings.value_color;
        state.semantic_color = Default::default();

        state.key.clear();
        state.key.push_str(&text); // FIXME: Uncow

        state.value.clear();
        let _ = write!(
            state.value,
            "{}",
            SegmentTime::with_accuracy(self.settings.accuracy).format(time)
        );

        state.key_abbreviations.clear();
        if self.settings.total_possible_time_save {
            state
                .key_abbreviations
                .push("Total Possible Time Save".into());
        }
        state.key_abbreviations.push("Possible Time Save".into());
        state.key_abbreviations.push("Poss. Time Save".into());
        state.key_abbreviations.push("Time Save".into());

        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently = updates_frequently;
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Snapshot<'_>) -> key_value::State {
        let mut state = Default::default();
        self.update_state(&mut state, timer);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Background".into(),
                "The background shown behind the component.".into(),
                self.settings.background.into(),
            ),
            Field::new(
                "Comparison".into(),
                "The comparison to calculate the possible time save for. If not specified, the current comparison is used.".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Display 2 Rows".into(),
                "Specifies whether to display the name of the component and the possible time save in two separate rows.".into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                "Show Total Possible Time Save".into(),
                "Specifies whether to show the total possible time save for the remainder of the current attempt, instead of the possible time save for the current segment.".into(),
                self.settings.total_possible_time_save.into(),
            ),
            Field::new(
                "Label Color".into(),
                "The color of the component's name. If not specified, the color is taken from the layout.".into(),
                self.settings.label_color.into(),
            ),
            Field::new(
                "Value Color".into(),
                "The color of the possible time save. If not specified, the color is taken from the layout.".into(),
                self.settings.value_color.into(),
            ),
            Field::new(
                "Accuracy".into(),
                "The accuracy of the possible time save shown.".into(),
                self.settings.accuracy.into(),
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
            1 => self.settings.comparison_override = value.into(),
            2 => self.settings.display_two_rows = value.into(),
            3 => self.settings.total_possible_time_save = value.into(),
            4 => self.settings.label_color = value.into(),
            5 => self.settings.value_color = value.into(),
            6 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
