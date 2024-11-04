//! Provides the `Timer` Component and relevant types for using it. The `Timer`
//! Component is a component that shows the total time of the current attempt as
//! a digital clock. The color of the time shown is based on a how well the
//! current attempt is doing compared to the chosen comparison.

use crate::{
    analysis::split_color,
    platform::prelude::*,
    settings::{Color, Field, Gradient, SemanticColor, SettingsDescription, Value},
    timing::{
        formatter::{timer as formatter, Accuracy, DigitsFormat, TimeFormatter},
        Snapshot,
    },
    GeneralLayoutSettings, TimeSpan, TimerPhase, TimingMethod,
};
use core::fmt::Write;
use serde_derive::{Deserialize, Serialize};

/// The `Timer` Component is a component that shows the total time of the current
/// attempt as a digital clock. The color of the time shown is based on a how
/// well the current attempt is doing compared to the chosen comparison.
#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

/// Represents the possible backgrounds for a timer.
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "serialize::DeltaGradient", into = "serialize::DeltaGradient")]
pub enum DeltaGradient {
    /// A normal gradient of some kind
    Gradient(Gradient),
    /// Delta based plain color
    DeltaPlain,
    /// Delta based gradient, Vertical
    DeltaVertical,
    /// Delta based gradient, horizontal
    DeltaHorizontal,
}

impl From<Gradient> for DeltaGradient {
    fn from(g: Gradient) -> Self {
        Self::Gradient(g)
    }
}

impl Default for DeltaGradient {
    fn default() -> Self {
        Self::Gradient(Gradient::default())
    }
}

impl DeltaGradient {
    /// Converts the DeltaGradient to a normal gradient for purposes of rendering
    ///
    ///  # Arguments
    /// * `delta` - the color used to represent the timer's current state
    pub fn gradient(&self, delta: Color) -> Gradient {
        let [h, s, v, a] = delta.to_hsva();

        match self {
            DeltaGradient::Gradient(g) => *g,
            DeltaGradient::DeltaVertical => {
                let color_a = Color::hsva(h, s * 0.5, v * 0.25, a * (1.0 / 6.0));
                let color_b = Color::hsva(h, s * 0.5, v * 0.25, a);

                Gradient::Vertical(color_a, color_b)
            }
            DeltaGradient::DeltaPlain => {
                Gradient::Plain(Color::hsva(h, s * 0.5, v * 0.25, a * (7.0 / 12.0)))
            }
            DeltaGradient::DeltaHorizontal => {
                let color_a = Color::hsva(h, s * 0.5, v * 0.25, a * (1.0 / 6.0));
                let color_b = Color::hsva(h, s * 0.5, v * 0.25, a);

                Gradient::Horizontal(color_a, color_b)
            }
        }
    }
}
/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The background shown behind the component.
    pub background: DeltaGradient,
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
    /// Specifies whether to show how much time has passed since the start
    /// current segment, rather than how much time has passed since the start of
    /// the current attempt.
    pub is_segment_timer: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: DeltaGradient::default(),
            timing_method: None,
            height: 60,
            color_override: None,
            show_gradient: true,
            digits_format: DigitsFormat::SingleDigitSeconds,
            accuracy: Accuracy::Hundredths,
            is_segment_timer: false,
        }
    }
}

/// The state object describes the information to visualize for this component.
#[derive(Default, Serialize, Deserialize)]
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
    /// This value indicates whether the timer is currently frequently being
    /// updated. This can be used for rendering optimizations.
    pub updates_frequently: bool,
}

#[cfg(feature = "std")]
impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}

impl Component {
    /// Creates a new Timer Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Timer Component with the given settings.
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
    pub const fn name(&self) -> &'static str {
        if self.settings.is_segment_timer {
            "Segment Timer"
        } else {
            "Timer"
        }
    }

    /// Updates the component's state based on the timer and layout settings
    /// provided.
    pub fn update_state(
        &self,
        state: &mut State,
        timer: &Snapshot<'_>,
        layout_settings: &GeneralLayoutSettings,
    ) {
        let method = self
            .settings
            .timing_method
            .unwrap_or_else(|| timer.current_timing_method());

        let phase = timer.current_phase();

        let (time, semantic_color) = if self.settings.is_segment_timer {
            let last_split_index = if phase == TimerPhase::Ended {
                timer.run().len() - 1
            } else {
                timer.current_split_index().unwrap_or_default()
            };
            let mut segment_time = calculate_live_segment_time(timer, method, last_split_index);

            if segment_time.is_none() && method == TimingMethod::GameTime {
                segment_time =
                    calculate_live_segment_time(timer, TimingMethod::RealTime, last_split_index);
            }

            (segment_time, SemanticColor::Default)
        } else {
            let time = timer.current_time();
            let time = time[method].or(time.real_time).unwrap_or_default();
            let current_comparison = timer.current_comparison();

            let semantic_color = match phase {
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
                        )
                        .or(SemanticColor::AheadGainingTime)
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

                    if pb_time.is_none_or(|t| time < t) {
                        SemanticColor::PersonalBest
                    } else {
                        SemanticColor::BehindLosingTime
                    }
                }
                _ => SemanticColor::NotRunning,
            };

            (Some(time), semantic_color)
        };

        let not_overwritten_visual_color = semantic_color.visualize(layout_settings);
        let visual_color = if let Some(color) = self.settings.color_override {
            color
        } else {
            not_overwritten_visual_color
        };

        (state.top_color, state.bottom_color) = if self.settings.show_gradient {
            top_and_bottom_color(visual_color)
        } else {
            (visual_color, visual_color)
        };

        state.background = self
            .settings
            .background
            .gradient(not_overwritten_visual_color);

        state.time.clear();
        let _ = write!(
            state.time,
            "{}",
            formatter::Time::with_digits_format(self.settings.digits_format).format(time),
        );

        state.fraction.clear();
        let _ = write!(
            state.fraction,
            "{}",
            formatter::Fraction::with_accuracy(self.settings.accuracy).format(time),
        );

        state.updates_frequently = phase.updates_frequently(method) && time.is_some();
        state.semantic_color = semantic_color;
        state.height = self.settings.height;
    }

    /// Calculates the component's state based on the timer and the layout
    /// settings provided.
    pub fn state(&self, timer: &Snapshot<'_>, layout_settings: &GeneralLayoutSettings) -> State {
        let mut state = Default::default();
        self.update_state(&mut state, timer, layout_settings);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Background".into(),
                "The background shown behind the component. It is also possible to apply the color associated with the time ahead or behind as the background color.".into(),
                self.settings.background.into(),
            ),
            Field::new(
                "Segment Timer".into(),
                "Specifies whether to show how much time has passed since the start of the current segment, rather than how much time has passed since the start of the current attempt.".into(),
                self.settings.is_segment_timer.into(),
            ),
            Field::new(
                "Timing Method".into(),
                "Specifies the timing method to use. If not specified, the current timing method is used.".into(),
                self.settings.timing_method.into(),
            ),
            Field::new(
                "Height".into(),
                "The height of the timer.".into(),
                u64::from(self.settings.height).into(),
            ),
            Field::new(
                "Text Color".into(),
                "The color of the time shown. If not specified, the color is automatically chosen based on how well the current attempt is going. Those colors can be specified in the general settings for the layout.".into(),
                self.settings.color_override.into(),
            ),
            Field::new(
                "Show Gradient".into(),
                "Determines whether to display the timer's color as a gradient.".into(),
                self.settings.show_gradient.into(),
            ),
            Field::new(
                "Digits Format".into(),
                "Specifies how many digits to show. If the duration is lower than the digits to be shown, zeros are shown instead.".into(),
                self.settings.digits_format.into(),
            ),
            Field::new(
                "Accuracy".into(),
                "The accuracy of the time shown.".into(),
                self.settings.accuracy.into(),
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
    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => self.settings.is_segment_timer = value.into(),
            2 => self.settings.timing_method = value.into(),
            3 => self.settings.height = value.into_uint().unwrap() as _,
            4 => self.settings.color_override = value.into(),
            5 => self.settings.show_gradient = value.into(),
            6 => self.settings.digits_format = value.into(),
            7 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}

/// Calculates the top and bottom color the Timer Component would use for the
/// gradient of the times it is showing.
pub fn top_and_bottom_color(color: Color) -> (Color, Color) {
    let [h, s, v, a] = color.to_hsva();

    let top_color = Color::hsva(h, 0.5 * s, (1.5 * v + 0.1).min(1.0), a);
    let bottom_color = Color::hsva(h, s, 0.8 * v, a);

    (top_color, bottom_color)
}

fn calculate_live_segment_time(
    timer: &Snapshot<'_>,
    timing_method: TimingMethod,
    last_split_index: usize,
) -> Option<TimeSpan> {
    let last_split = if last_split_index > 0 {
        timer.run().segment(last_split_index - 1).split_time()[timing_method]
    } else {
        Some(TimeSpan::zero())
    };

    if timer.current_phase() == TimerPhase::NotRunning {
        Some(timer.run().offset())
    } else {
        Some(timer.current_time()[timing_method]? - last_split?)
    }
}

// FIXME: Workaround for #[serde(flatten)] not being a thing on enums.
mod serialize {
    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(untagged)]
    pub enum DeltaGradient {
        Gradient(super::Gradient),
        Delta(Delta),
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[allow(clippy::enum_variant_names)]
    pub enum Delta {
        DeltaPlain,
        DeltaVertical,
        DeltaHorizontal,
    }

    impl From<DeltaGradient> for super::DeltaGradient {
        fn from(v: DeltaGradient) -> Self {
            match v {
                DeltaGradient::Gradient(g) => super::DeltaGradient::Gradient(g),
                DeltaGradient::Delta(d) => match d {
                    Delta::DeltaPlain => super::DeltaGradient::DeltaPlain,
                    Delta::DeltaVertical => super::DeltaGradient::DeltaVertical,
                    Delta::DeltaHorizontal => super::DeltaGradient::DeltaHorizontal,
                },
            }
        }
    }

    impl From<super::DeltaGradient> for DeltaGradient {
        fn from(v: super::DeltaGradient) -> Self {
            match v {
                super::DeltaGradient::Gradient(g) => DeltaGradient::Gradient(g),
                super::DeltaGradient::DeltaPlain => DeltaGradient::Delta(Delta::DeltaPlain),
                super::DeltaGradient::DeltaVertical => DeltaGradient::Delta(Delta::DeltaVertical),
                super::DeltaGradient::DeltaHorizontal => {
                    DeltaGradient::Delta(Delta::DeltaHorizontal)
                }
            }
        }
    }
}
