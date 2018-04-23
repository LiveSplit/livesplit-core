//! Provides the Timer Component and relevant types for using it. The Timer
//! Component is a component that shows the total time of the current attempt as
//! a digital clock. The color of the time shown is based on a how well the
//! current attempt is doing compared to the chosen comparison.

use analysis::split_color;
use palette::rgb::Rgb;
use palette::Hsv;
use serde_json::{to_writer, Result};
use settings::{Color, Field, Gradient, SemanticColor, SettingsDescription, Value};
use std::borrow::Cow;
use std::io::Write;
use time::formatter::{timer as formatter, Accuracy, DigitsFormat, TimeFormatter};
use {GeneralLayoutSettings, TimeSpan, Timer, TimerPhase, TimingMethod};

/// The Timer Component is a component that shows the total time of the current
/// attempt as a digital clock. The color of the time shown is based on a how
/// well the current attempt is doing compared to the chosen comparison.
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
    /// Specifies the Timing Method to use. If set to `None` the Timing Method
    /// of the Timer is used for showing the time. Otherwise the Timing Method
    /// provided is used.
    pub timing_method: Option<TimingMethod>,
    /// The height of the timer.
    pub height: u32,
    /// Instead of automatically determining the color of the time shown, based
    /// on a how well the current attempt is doing, a specific color to always
    /// be used can be provided instead.
    pub color_override: Option<Color>,
    /// The Timer Component automatically converts the color it is supposed to
    /// use into a gradient and shows that. If this is set to `false` the actual
    /// color is used instead of a gradient.
    pub show_gradient: bool,
    /// Determines how many digits are to always be shown. If the duration is
    /// lower than the digits to be shown, they are filled up with zeros.
    pub digits_format: DigitsFormat,
    /// The accuracy of the time shown.
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Gradient::Transparent,
            timing_method: None,
            height: 60,
            color_override: None,
            show_gradient: true,
            digits_format: DigitsFormat::SingleDigitSeconds,
            accuracy: Accuracy::Hundredths,
        }
    }
}

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The time shown by the component without the fractional part.
    pub time: String,
    /// The fractional part of the time shown (including the dot).
    pub fraction: String,
    /// The semantic coloring information the time carries.
    pub semantic_color: SemanticColor,
    /// The top color of the timer's gradient.
    pub top_color: Color,
    /// The bottom color of the timer's gradient.
    pub bottom_color: Color,
    /// The height of the timer.
    pub height: u32,
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
    /// Creates a new Timer Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Timer Component with the given settings.
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
        "Timer".into()
    }

    /// Calculates the component's state based on the timer and the layout
    /// settings provided.
    pub fn state(&self, timer: &Timer, layout_settings: &GeneralLayoutSettings) -> State {
        let method = self.settings
            .timing_method
            .unwrap_or_else(|| timer.current_timing_method());
        let time = timer.current_time();
        let time = time[method].or(time.real_time).unwrap_or_default();
        let current_comparison = timer.current_comparison();

        let semantic_color = match timer.current_phase() {
            TimerPhase::Running if time >= TimeSpan::zero() => {
                let pb_split_time = timer
                    .current_split()
                    .unwrap()
                    .comparison(current_comparison)[method];

                if let Some(pb_split_time) = pb_split_time {
                    split_color(
                        timer,
                        Some(time - pb_split_time),
                        timer.current_split_index().unwrap(),
                        true,
                        false,
                        current_comparison,
                        method,
                    ).or(SemanticColor::AheadGainingTime)
                } else {
                    SemanticColor::AheadGainingTime
                }
            }
            TimerPhase::Paused => SemanticColor::Paused,
            TimerPhase::Ended => {
                let pb_time = timer
                    .run()
                    .segments()
                    .last()
                    .unwrap()
                    .comparison(current_comparison)[method];

                if pb_time.map_or(true, |t| time < t) {
                    SemanticColor::PersonalBest
                } else {
                    SemanticColor::BehindLosingTime
                }
            }
            _ => SemanticColor::NotRunning,
        };

        let visual_color = if let Some(color) = self.settings.color_override {
            color
        } else {
            semantic_color.visualize(layout_settings)
        };

        let (top_color, bottom_color) = if self.settings.show_gradient {
            top_and_bottom_color(visual_color)
        } else {
            (visual_color, visual_color)
        };

        State {
            background: self.settings.background,
            time: formatter::Time::with_digits_format(self.settings.digits_format)
                .format(time)
                .to_string(),
            fraction: formatter::Fraction::with_accuracy(self.settings.accuracy)
                .format(time)
                .to_string(),
            semantic_color,
            top_color,
            bottom_color,
            height: self.settings.height,
        }
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new("Timing Method".into(), self.settings.timing_method.into()),
            Field::new("Height".into(), (self.settings.height as u64).into()),
            Field::new("Text Color".into(), self.settings.color_override.into()),
            Field::new("Show Gradient".into(), self.settings.show_gradient.into()),
            Field::new("Digits Format".into(), self.settings.digits_format.into()),
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
            1 => self.settings.timing_method = value.into(),
            2 => self.settings.height = value.into_uint().unwrap() as _,
            3 => self.settings.color_override = value.into(),
            4 => self.settings.show_gradient = value.into(),
            5 => self.settings.digits_format = value.into(),
            6 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}

/// Calculates the top and bottom color the Timer Component would use for the
/// gradient of the times it is showing.
pub fn top_and_bottom_color(color: Color) -> (Color, Color) {
    let hsv: Hsv = color.rgba.into();

    let h = hsv.hue.to_degrees() as f64;
    let s = hsv.saturation as f64;
    let v = hsv.value as f64;
    let a = color.rgba.alpha;

    let top_color = Rgb::from(Hsv::new(h.into(), 0.5 * s, (1.5 * v + 0.1).min(1.0)));
    let bottom_color = Rgb::from(Hsv::new(h.into(), s, 0.8 * v));

    let top_color = Color::from((
        top_color.red as f32,
        top_color.green as f32,
        top_color.blue as f32,
        a,
    ));

    let bottom_color = Color::from((
        bottom_color.red as f32,
        bottom_color.green as f32,
        bottom_color.blue as f32,
        a,
    ));

    (top_color, bottom_color)
}
