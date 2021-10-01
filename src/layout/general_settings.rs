use super::LayoutDirection;
use crate::{
    platform::prelude::*,
    settings::{Color, Field, Font, Gradient, SettingsDescription, Value},
    timing::formatter::Accuracy,
};
use serde::{Deserialize, Serialize};

/// The general settings of the layout that apply to all components.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralSettings {
    /// The direction which the components are laid out in.
    pub direction: LayoutDirection,
    /// The font to use for the timer text. `None` means a default font should
    /// be used.
    pub timer_font: Option<Font>,
    /// The font to use for the times and other values. `None` means a default
    /// font should be used.
    pub times_font: Option<Font>,
    /// The font to use for regular text. `None` means a default font should be
    /// used.
    pub text_font: Option<Font>,
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
    /// Specifies the display accuracy of split times.
    pub split_time_accuracy: Accuracy,
    /// Specifies the display accuracy of segment times.
    pub segment_time_accuracy: Accuracy,
    /// Specifies the display accuracy of delta times.
    pub delta_time_accuracy: Accuracy,
    /// Whether to drop the fractional part of a delta time once it goes past
    /// one minute.
    pub delta_drop_decimals: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            timer_font: None,
            times_font: None,
            text_font: None,
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
            split_time_accuracy: Accuracy::Seconds,
            segment_time_accuracy: Accuracy::Hundredths,
            delta_time_accuracy: Accuracy::Tenths,
            delta_drop_decimals: true,
        }
    }
}

impl GeneralSettings {
    /// Accesses a generic description of the general settings available for the
    /// layout and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Layout Direction".into(), self.direction.into()),
            Field::new("Custom Timer Font".into(), self.timer_font.clone().into()),
            Field::new("Custom Times Font".into(), self.times_font.clone().into()),
            Field::new("Custom Text Font".into(), self.text_font.clone().into()),
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
            1 => self.timer_font = value.into(),
            2 => self.times_font = value.into(),
            3 => self.text_font = value.into(),
            4 => self.background = value.into(),
            5 => self.best_segment_color = value.into(),
            6 => self.ahead_gaining_time_color = value.into(),
            7 => self.ahead_losing_time_color = value.into(),
            8 => self.behind_gaining_time_color = value.into(),
            9 => self.behind_losing_time_color = value.into(),
            10 => self.not_running_color = value.into(),
            11 => self.personal_best_color = value.into(),
            12 => self.paused_color = value.into(),
            13 => self.thin_separators_color = value.into(),
            14 => self.separators_color = value.into(),
            15 => self.text_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
