//! Provides the Current Pace Component and relevant types for using it. The
//! Current Pace Component is a component that shows a prediction of the current
//! attempt's final time, if the current attempt's pace matches the chosen
//! comparison for the remainder of the run.

use super::key_value;
use crate::analysis::current_pace;
use crate::platform::prelude::*;
use crate::settings::{Color, Field, Gradient, SettingsDescription, Value};
use crate::timing::formatter::{Accuracy, Regular, TimeFormatter};
use crate::{comparison, Timer, TimerPhase};
use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};

/// The Current Pace Component is a component that shows a prediction of the
/// current attempt's final time, if the current attempt's pace matches the
/// chosen comparison for the remainder of the run.
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
            label_color: None,
            value_color: None,
            accuracy: Accuracy::Seconds,
        }
    }
}

impl Component {
    /// Creates a new Current Pace Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Current Pace Component with the given settings.
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
                .map(String::as_str),
        )
    }

    fn text(&self, comparison: Option<&str>) -> Cow<'static, str> {
        if let Some(comparison) = comparison {
            match comparison {
                comparison::personal_best::NAME => "Current Pace".into(),
                comparison::best_segments::NAME => "Best Possible Time".into(),
                comparison::worst_segments::NAME => "Worst Possible Time".into(),
                comparison::average_segments::NAME => "Predicted Time".into(),
                comparison => format!("Current Pace ({})", comparison::shorten(comparison)).into(),
            }
        } else {
            "Current Pace".into()
        }
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Timer) -> key_value::State {
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(comparison, timer);
        let key = self.text(Some(comparison)).into_owned();

        let current_pace =
            if timer.current_phase() == TimerPhase::NotRunning && key.starts_with("Current Pace") {
                None
            } else {
                current_pace::calculate(timer, comparison)
            };

        let key_abbreviations = match &*key {
            "Best Possible Time" => {
                Box::new(["Best Poss. Time".into(), "Best Time".into(), "BPT".into()]) as _
            }
            "Worst Possible Time" => {
                Box::new(["Worst Poss. Time".into(), "Worst Time".into()]) as _
            }
            "Predicted Time" => Box::new(["Pred. Time".into()]) as _,
            "Current Pace" => Box::new(["Cur. Pace".into(), "Pace".into()]) as _,
            _ => Box::new(["Current Pace".into(), "Cur. Pace".into(), "Pace".into()]) as _,
        };

        key_value::State {
            background: self.settings.background,
            key_color: self.settings.label_color,
            value_color: self.settings.value_color,
            semantic_color: Default::default(),
            key: key.into(),
            value: Regular::with_accuracy(self.settings.accuracy)
                .format(current_pace)
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
            3 => self.settings.label_color = value.into(),
            4 => self.settings.value_color = value.into(),
            5 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
