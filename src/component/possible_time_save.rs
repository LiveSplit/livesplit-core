//! Provides the Possible Time Save Component and relevant types for using it.
//! The Possible Time Save Component is a component that shows how much time the
//! chosen comparison could've saved for the current segment, based on the Best
//! Segments. This component also allows showing the Total Possible Time Save
//! for the remainder of the current attempt.

use super::key_value;
use crate::analysis::possible_time_save;
use crate::settings::{Color, Field, Gradient, SettingsDescription, Value};
use crate::timing::formatter::{Accuracy, PossibleTimeSave, TimeFormatter};
use crate::{comparison, Timer, TimerPhase};
use serde::{Deserialize, Serialize};
use alloc::borrow::Cow;
use core::fmt::Write as FmtWrite;
use crate::platform::prelude::*;

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
    pub fn with_settings(settings: Settings) -> Self {
        Self { settings }
    }

    /// Accesses the settings of the component.
    pub fn settings(&self) -> &Settings {
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

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Timer) -> key_value::State {
        let segment_index = timer.current_split_index();
        let current_phase = timer.current_phase();
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let text = self.text(comparison);
        let comparison = comparison::or_current(comparison, timer);

        let time = if self.settings.total_possible_time_save {
            Some(possible_time_save::calculate_total(
                timer,
                segment_index.unwrap_or(0),
                comparison,
            ))
        } else if current_phase == TimerPhase::Running || current_phase == TimerPhase::Paused {
            possible_time_save::calculate(timer, segment_index.unwrap(), comparison, false)
        } else {
            None
        };

        let key_abbreviations = if self.settings.total_possible_time_save {
            Box::new([
                "Total Possible Time Save".into(),
                "Possible Time Save".into(),
                "Poss. Time Save".into(),
                "Time Save".into(),
            ]) as _
        } else {
            Box::new([
                "Possible Time Save".into(),
                "Poss. Time Save".into(),
                "Time Save".into(),
            ]) as _
        };

        key_value::State {
            background: self.settings.background,
            key_color: self.settings.label_color,
            value_color: self.settings.value_color,
            semantic_color: Default::default(),
            key: text.into_owned().into(),
            value: PossibleTimeSave::with_accuracy(self.settings.accuracy)
                .format(time)
                .to_string()
                .into(),
            key_abbreviations,
            display_two_rows: self.settings.display_two_rows,
        }
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new(
                "Comparison".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Display 2 Rows".into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                "Show Total Possible Time Save".into(),
                self.settings.total_possible_time_save.into(),
            ),
            Field::new("Label Color".into(), self.settings.label_color.into()),
            Field::new("Value Color".into(), self.settings.value_color.into()),
            Field::new("Accuracy".into(), self.settings.accuracy.into()),
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
