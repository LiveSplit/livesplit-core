use super::LayoutDirection;
use crate::{
    platform::prelude::*,
    settings::{
        Color, Field, Font, Gradient, ImageCache, LayoutBackground, SettingsDescription, Value,
    },
};
use serde_derive::{Deserialize, Serialize};

/// The general settings of a [`Layout`](crate::layout::Layout) that apply to all components.
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
    pub background: LayoutBackground,
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
    /// Ignore Mouse While Running and Not In Focus
    pub mouse_pass_through_while_running: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            timer_font: None,
            times_font: None,
            text_font: None,
            background: LayoutBackground::Gradient(Gradient::Plain(Color::hsla(
                0.0, 0.0, 0.06, 1.0,
            ))),
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
            mouse_pass_through_while_running: false,
        }
    }
}

impl GeneralSettings {
    /// Accesses a generic description of the general settings available for the
    /// layout and their current values. The [`ImageCache`] is updated with all
    /// the images that are part of the state. The images are marked as visited
    /// in the [`ImageCache`]. You still need to manually run
    /// [`ImageCache::collect`] to ensure unused images are removed from the
    /// cache.
    pub fn settings_description(&self, image_cache: &mut ImageCache) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Layout Direction".into(),
                "The direction in which the components are laid out.".into(),
                self.direction.into(),
            ),
            Field::new(
                "Custom Timer Font".into(),
                "Allows you to specify a custom font for the timer. If this is not set, the default font is used."
                    .into(),
                self.timer_font.clone().into(),
            ),
            Field::new(
                "Custom Times Font".into(),
                "Allows you to specify a custom font for the times. If this is not set, the default font is used."
                    .into(),
                self.times_font.clone().into(),
            ),
            Field::new(
                "Custom Text Font".into(),
                "Allows you to specify a custom font for the text. If this is not set, the default font is used."
                    .into(),
                self.text_font.clone().into(),
            ),
            Field::new(
                "Background".into(),
                "The background shown behind the entire layout.".into(),
                self.background.cache(image_cache).into(),
            ),
            Field::new(
                "Best Segment".into(),
                "The color to use for when you achieve a new best segment.".into(),
                self.best_segment_color.into(),
            ),
            Field::new(
                "Ahead (Gaining Time)".into(),
                "The color to use for when you are ahead of the comparison and are gaining even more time."
                    .into(),
                self.ahead_gaining_time_color.into(),
            ),
            Field::new(
                "Ahead (Losing Time)".into(),
                "The color to use for when you are ahead of the comparison, but are losing time."
                    .into(),
                self.ahead_losing_time_color.into(),
            ),
            Field::new(
                "Behind (Gaining Time)".into(),
                "The color to use for when you are behind the comparison, but are gaining back time."
                    .into(),
                self.behind_gaining_time_color.into(),
            ),
            Field::new(
                "Behind (Losing Time)".into(),
                "The color to use for when you are behind the comparison and are losing even more time."
                    .into(),
                self.behind_losing_time_color.into(),
            ),
            Field::new(
                "Not Running".into(),
                "The color to use for when there is no active attempt.".into(),
                self.not_running_color.into(),
            ),
            Field::new(
                "Personal Best".into(),
                "The color to use for when you achieve a new Personal Best.".into(),
                self.personal_best_color.into(),
            ),
            Field::new(
                "Paused".into(),
                "The color to use for when the timer is paused.".into(),
                self.paused_color.into(),
            ),
            Field::new(
                "Thin Separators".into(),
                "The color of thin separators.".into(),
                self.thin_separators_color.into(),
            ),
            Field::new(
                "Separators".into(),
                "The color of normal separators.".into(),
                self.separators_color.into(),
            ),
            Field::new(
                "Text".into(),
                "The color to use for text that doesn't specify its own color.".into(),
                self.text_color.into(),
            ),
            Field::new(
                "Running Ignore Mouse".into(),
                "Ignore Mouse While Running and Not In Focus".into(),
                self.mouse_pass_through_while_running.into(),
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
    pub fn set_value(&mut self, index: usize, value: Value, image_cache: &ImageCache) {
        match index {
            0 => self.direction = value.into(),
            1 => self.timer_font = value.into(),
            2 => self.times_font = value.into(),
            3 => self.text_font = value.into(),
            4 => self.background = LayoutBackground::from(value).from_cache(image_cache),
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
            16 => self.mouse_pass_through_while_running = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
