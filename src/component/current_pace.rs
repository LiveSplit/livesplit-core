//! Provides the Current Pace Component and relevant types for using it. The
//! Current Pace Component is a component that shows a prediction of the current
//! attempt's final time, if the current attempt's pace matches the chosen
//! comparison for the remainder of the run.

use super::DEFAULT_INFO_TEXT_GRADIENT;
use analysis::current_pace;
use serde_json::{to_writer, Result};
use settings::{Color, Field, Gradient, SettingsDescription, Value};
use std::borrow::Cow;
use std::io::Write;
use time::formatter::{Accuracy, Regular, TimeFormatter};
use {comparison, Timer, TimerPhase};

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
            background: DEFAULT_INFO_TEXT_GRADIENT,
            comparison_override: None,
            label_color: None,
            value_color: None,
            accuracy: Accuracy::Seconds,
        }
    }
}

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The color of the label. If `None` is specified, the color is taken from
    /// the layout.
    pub label_color: Option<Color>,
    /// The color of the value. If `None` is specified, the color is taken from
    /// the layout.
    pub value_color: Option<Color>,
    /// The label's text.
    pub text: String,
    /// The current pace.
    pub time: String,
}

impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Component {
    /// Creates a new Current Pace Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Current Pace Component with the given settings.
    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
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
    pub fn name(&self) -> Cow<str> {
        self.text(
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_str),
        )
    }

    fn text(&self, comparison: Option<&str>) -> Cow<str> {
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
    pub fn state(&self, timer: &Timer) -> State {
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(comparison, timer);
        let text = self.text(Some(comparison)).into_owned();

        let current_pace = if timer.current_phase() == TimerPhase::NotRunning
            && text.starts_with("Current Pace")
        {
            None
        } else {
            current_pace::calculate(timer, comparison)
        };

        State {
            background: self.settings.background,
            label_color: self.settings.label_color,
            value_color: self.settings.value_color,
            text,
            time: Regular::with_accuracy(self.settings.accuracy)
                .format(current_pace)
                .to_string(),
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
            2 => self.settings.label_color = value.into(),
            3 => self.settings.value_color = value.into(),
            4 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
