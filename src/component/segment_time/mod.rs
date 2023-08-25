//! Provides the Segment Time Component. The Segment Time Component is a
//! component that shows the time for the current segment in a comparison of
//! your choosing. If no comparison is specified it uses the timer's current
//! comparison.

use super::key_value;
use crate::{
    analysis::state_helper::comparison_single_segment_time,
    comparison,
    platform::prelude::*,
    settings::{Color, Field, Gradient, SettingsDescription, Value},
    timing::formatter::{Accuracy, SegmentTime, TimeFormatter},
    Timer, TimerPhase,
};
use alloc::borrow::Cow;
use core::fmt::Write;
use serde_derive::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The Segment Time Component is a component that shows the time for the current
/// segment in a comparison of your choosing. If no comparison is specified it
/// uses the timer's current comparison.
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
            accuracy: Accuracy::Hundredths,
        }
    }
}

impl Component {
    /// Creates a new Segment Time Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Segment Time Component with the given settings.
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
        self.text(self.settings.comparison_override.as_deref())
    }

    fn text(&self, comparison: Option<&str>) -> Cow<'static, str> {
        if let Some(comparison) = comparison {
            match comparison {
                comparison::best_segments::NAME => "Best Segment Time".into(),
                comparison::worst_segments::NAME => "Worst Segment Time".into(),
                comparison::average_segments::NAME => "Average Segment Time".into(),
                comparison::median_segments::NAME => "Median Segment Time".into(),
                comparison::latest_run::NAME => "Latest Segment Time".into(),
                comparison => format!("Segment Time ({})", comparison::shorten(comparison)).into(),
            }
        } else {
            "Segment Time".into()
        }
    }

    /// Updates the component's state based on the timer provided.
    pub fn update_state(&self, state: &mut key_value::State, timer: &Timer) {
        let resolved_comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(resolved_comparison, timer);
        let key = self.text(resolved_comparison); // FIXME: Uncow

        let time = catch! {
            // FIXME: We shouldn't need to manually do this "bounds check".
            if timer.current_phase() == TimerPhase::Ended {
                return None;
            }

            comparison_single_segment_time(
                timer.run(),
                timer.current_split_index()?,
                comparison,
                timer.current_timing_method(),
            )?
        };

        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = self.settings.value_color;
        state.semantic_color = Default::default();

        state.key.clear();
        state.key.push_str(&key);

        state.value.clear();
        let _ = write!(
            state.value,
            "{}",
            SegmentTime::with_accuracy(self.settings.accuracy).format(time),
        );

        state.key_abbreviations.clear();
        match &*key {
            "Best Segment Time" => {
                state.key_abbreviations.push("Best Seg. Time".into());
                state.key_abbreviations.push("Best Segment".into());
            }
            "Worst Segment Time" => {
                state.key_abbreviations.push("Worst Seg. Time".into());
                state.key_abbreviations.push("Worst Segment".into());
            }
            "Average Segment Time" => {
                state.key_abbreviations.push("Average Seg. Time".into());
                state.key_abbreviations.push("Average Segment".into());
            }
            "Median Segment Time" => {
                state.key_abbreviations.push("Median Seg. Time".into());
                state.key_abbreviations.push("Median Segment".into());
            }
            "Latest Segment Time" => {
                state.key_abbreviations.push("Latest Seg. Time".into());
                state.key_abbreviations.push("Latest Segment".into());
            }
            "Segment Time" => state.key_abbreviations.push("Seg. Time".into()),
            _ => {
                state.key_abbreviations.push("Segment Time".into());
                state.key_abbreviations.push("Seg. Time".into());
            }
        };

        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently = false;
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Timer) -> key_value::State {
        let mut state = Default::default();
        self.update_state(&mut state, timer);
        state
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
