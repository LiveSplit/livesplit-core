use super::LayoutDirection;
use crate::{
    localization::{Lang, Text},
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
    /// The color to use for drawn shadows.
    pub text_shadow: Option<Color>,
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
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            timer_font: None,
            times_font: None,
            text_font: None,
            text_shadow: Some(Color::hsla(0.0, 0.0, 0.0, 0.5)),
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
        }
    }
}

impl GeneralSettings {
    /// Accesses a generic description of the general settings available for the
    /// layout and their current values for the specified language. The
    /// [`ImageCache`] is updated with all the images that are part of the state.
    /// The images are marked as visited in the [`ImageCache`]. You still need to
    /// manually run [`ImageCache::collect`] to ensure unused images are removed
    /// from the cache.
    pub fn settings_description(
        &self,
        image_cache: &mut ImageCache,
        lang: Lang,
    ) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                Text::LayoutDirection.resolve(lang).into(),
                Text::LayoutDirectionDescription.resolve(lang).into(),
                self.direction.into(),
            ),
            Field::new(
                Text::CustomTimerFont.resolve(lang).into(),
                Text::CustomTimerFontDescription.resolve(lang).into(),
                self.timer_font.clone().into(),
            ),
            Field::new(
                Text::CustomTimesFont.resolve(lang).into(),
                Text::CustomTimesFontDescription.resolve(lang).into(),
                self.times_font.clone().into(),
            ),
            Field::new(
                Text::CustomTextFont.resolve(lang).into(),
                Text::CustomTextFontDescription.resolve(lang).into(),
                self.text_font.clone().into(),
            ),
            Field::new(
                Text::TextShadow.resolve(lang).into(),
                Text::TextShadowDescription.resolve(lang).into(),
                self.text_shadow.into(),
            ),
            Field::new(
                Text::Background.resolve(lang).into(),
                Text::BackgroundDescription.resolve(lang).into(),
                self.background.cache(image_cache).into(),
            ),
            Field::new(
                Text::BestSegment.resolve(lang).into(),
                Text::BestSegmentDescription.resolve(lang).into(),
                self.best_segment_color.into(),
            ),
            Field::new(
                Text::AheadGainingTime.resolve(lang).into(),
                Text::AheadGainingTimeDescription.resolve(lang).into(),
                self.ahead_gaining_time_color.into(),
            ),
            Field::new(
                Text::AheadLosingTime.resolve(lang).into(),
                Text::AheadLosingTimeDescription.resolve(lang).into(),
                self.ahead_losing_time_color.into(),
            ),
            Field::new(
                Text::BehindGainingTime.resolve(lang).into(),
                Text::BehindGainingTimeDescription.resolve(lang).into(),
                self.behind_gaining_time_color.into(),
            ),
            Field::new(
                Text::BehindLosingTime.resolve(lang).into(),
                Text::BehindLosingTimeDescription.resolve(lang).into(),
                self.behind_losing_time_color.into(),
            ),
            Field::new(
                Text::NotRunning.resolve(lang).into(),
                Text::NotRunningDescription.resolve(lang).into(),
                self.not_running_color.into(),
            ),
            Field::new(
                Text::PersonalBest.resolve(lang).into(),
                Text::PersonalBestDescription.resolve(lang).into(),
                self.personal_best_color.into(),
            ),
            Field::new(
                Text::Paused.resolve(lang).into(),
                Text::PausedDescription.resolve(lang).into(),
                self.paused_color.into(),
            ),
            Field::new(
                Text::ThinSeparators.resolve(lang).into(),
                Text::ThinSeparatorsDescription.resolve(lang).into(),
                self.thin_separators_color.into(),
            ),
            Field::new(
                Text::Separators.resolve(lang).into(),
                Text::SeparatorsDescription.resolve(lang).into(),
                self.separators_color.into(),
            ),
            Field::new(
                Text::TextColor.resolve(lang).into(),
                Text::TextColorDescription.resolve(lang).into(),
                self.text_color.into(),
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
            4 => self.text_shadow = value.into(),
            5 => self.background = LayoutBackground::from(value).from_cache(image_cache),
            6 => self.best_segment_color = value.into(),
            7 => self.ahead_gaining_time_color = value.into(),
            8 => self.ahead_losing_time_color = value.into(),
            9 => self.behind_gaining_time_color = value.into(),
            10 => self.behind_losing_time_color = value.into(),
            11 => self.not_running_color = value.into(),
            12 => self.personal_best_color = value.into(),
            13 => self.paused_color = value.into(),
            14 => self.thin_separators_color = value.into(),
            15 => self.separators_color = value.into(),
            16 => self.text_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
