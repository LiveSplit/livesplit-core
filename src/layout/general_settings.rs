use super::LayoutDirection;
use crate::platform::prelude::*;
use crate::settings::{Color, Field, Gradient, SettingsDescription, Value};
use serde::{Deserialize, Serialize};

/// The general settings of the layout that apply to all components.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralSettings {
    /// The direction which the components are laid out in.
    pub direction: LayoutDirection,
    /// The background to show behind the layout.
    pub background: Gradient,
    /// The color to use for when the runner achieved a best segment.
    pub best_segment_color: Color,
    /// The color to use for when the runner is ahead of the comparison and is
    /// gaining even more time.
    pub ahead_gaining_time_color: Color,
    /// The color to use for when the runner is ahead of the comparison, but is
    /// losing time.
    pub ahead_losing_time_color: Color,
    /// The color to use for when the runner is behind the comparison, but is
    /// gaining back time.
    pub behind_gaining_time_color: Color,
    /// The color to use for when the runner is behind the comparison and is
    /// losing even more time.
    pub behind_losing_time_color: Color,
    /// The color to use for when there is no active attempt.
    pub not_running_color: Color,
    /// The color to use for when the runner achieved a new Personal Best.
    pub personal_best_color: Color,
    /// The color to use for when the timer is paused.
    pub paused_color: Color,
    /// The color of thin separators.
    pub thin_separators_color: Color,
    /// The color of normal separators.
    pub separators_color: Color,
    /// The text color to use for text that doesn't specify its own color.
    pub text_color: Color,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            background: Gradient::Plain(Color::hsla(0.0, 0.0, 0.06, 1.0)),
            best_segment_color: Color::hsla(50.0, 1.0, 0.5, 1.0),
            ahead_gaining_time_color: Color::hsla(136.0, 1.0, 0.4, 1.0),
            ahead_losing_time_color: Color::hsla(136.0, 0.55, 0.6, 1.0),
            behind_gaining_time_color: Color::hsla(0.0, 0.55, 0.6, 1.0),
            behind_losing_time_color: Color::hsla(0.0, 1.0, 0.4, 1.0),
            not_running_color: Color::hsla(0.0, 0.0, 0.67, 1.0),
            personal_best_color: Color::hsla(203.0, 1.0, 0.54, 1.0),
            paused_color: Color::hsla(0.0, 0.0, 0.48, 1.0),
            thin_separators_color: Color::hsla(0.0, 0.0, 1.0, 0.09),
            separators_color: Color::hsla(0.0, 0.0, 1.0, 0.35),
            text_color: Color::hsla(0.0, 0.0, 1.0, 1.0),
        }
    }
}

impl GeneralSettings {
    /// Accesses a generic description of the general settings available for the
    /// layout and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Layout Direction".into(), self.direction.into()),
            Field::new("Background".into(), self.background.into()),
            Field::new("Best Segment".into(), self.best_segment_color.into()),
            Field::new(
                "Ahead (Gaining Time)".into(),
                self.ahead_gaining_time_color.into(),
            ),
            Field::new(
                "Ahead (Losing Time)".into(),
                self.ahead_losing_time_color.into(),
            ),
            Field::new(
                "Behind (Gaining Time)".into(),
                self.behind_gaining_time_color.into(),
            ),
            Field::new(
                "Behind (Losing Time)".into(),
                self.behind_losing_time_color.into(),
            ),
            Field::new("Not Running".into(), self.not_running_color.into()),
            Field::new("Personal Best".into(), self.personal_best_color.into()),
            Field::new("Paused".into(), self.paused_color.into()),
            Field::new("Thin Separators".into(), self.thin_separators_color.into()),
            Field::new("Separators".into(), self.separators_color.into()),
            Field::new("Text".into(), self.text_color.into()),
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
            0 => self.direction = value.into(),
            1 => self.background = value.into(),
            2 => self.best_segment_color = value.into(),
            3 => self.ahead_gaining_time_color = value.into(),
            4 => self.ahead_losing_time_color = value.into(),
            5 => self.behind_gaining_time_color = value.into(),
            6 => self.behind_losing_time_color = value.into(),
            7 => self.not_running_color = value.into(),
            8 => self.personal_best_color = value.into(),
            9 => self.paused_color = value.into(),
            10 => self.thin_separators_color = value.into(),
            11 => self.separators_color = value.into(),
            12 => self.text_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
