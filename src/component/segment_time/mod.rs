//! Provides the Segment Time Component. The Segment Time Component is a
//! component that shows the time for the current segment in a comparison of
//! your choosing. If no comparison is specified it uses the timer's current
//! comparison.

use super::key_value;
use crate::analysis::state_helper::comparison_single_segment_time;
use crate::platform::prelude::*;
use crate::settings::{Color, Field, Gradient, SettingsDescription, Value};
use crate::timing::formatter::{Accuracy, SegmentTime, TimeFormatter};
use crate::{comparison, Timer, TimerPhase};
use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};

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

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Timer) -> key_value::State {
        let resolved_comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(resolved_comparison, timer);
        let key = self.text(resolved_comparison).into_owned();

        let key_abbreviations = match &*key {
            "Best Segment Time" => Box::new(["Best Seg. Time".into(), "Best Segment".into()]) as _,
            "Worst Segment Time" => {
                Box::new(["Worst Seg. Time".into(), "Worst Segment".into()]) as _
            }
            "Average Segment Time" => {
                Box::new(["Average Seg. Time".into(), "Average Segment".into()]) as _
            }
            "Median Segment Time" => {
                Box::new(["Median Seg. Time".into(), "Median Segment".into()]) as _
            }
            "Latest Segment Time" => {
                Box::new(["Latest Seg. Time".into(), "Latest Segment".into()]) as _
            }
            "Segment Time" => Box::new(["Seg. Time".into()]) as _,
            _ => Box::new(["Segment Time".into(), "Seg. Time".into()]) as _,
        };

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

        key_value::State {
            background: self.settings.background,
            key_color: self.settings.label_color,
            value_color: self.settings.value_color,
            semantic_color: Default::default(),
            key: key.into(),
            value: SegmentTime::with_accuracy(self.settings.accuracy)
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
