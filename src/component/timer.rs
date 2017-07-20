use {SemanticColor, Color, GeneralLayoutSettings, Timer, TimerPhase, TimeSpan, TimingMethod};
use time_formatter::{timer as formatter, TimeFormatter, Accuracy, DigitsFormat};
use analysis::split_color;
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use layout::editor::{SettingsDescription, Value, Field};
use palette::{Hsva, Rgba};

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub timing_method: Option<TimingMethod>,
    pub digits_format: DigitsFormat,
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            timing_method: None,
            digits_format: DigitsFormat::SingleDigitSeconds,
            accuracy: Accuracy::Hundredths,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub time: String,
    pub fraction: String,
    pub semantic_color: SemanticColor,
    pub top_color: Color,
    pub bottom_color: Color,
}

impl State {
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn name(&self) -> Cow<str> {
        "Timer".into()
    }

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
                    .comparison(current_comparison)
                    [method];

                if let Some(pb_split_time) = pb_split_time {
                    split_color(
                        timer,
                        Some(time - pb_split_time),
                        timer.current_split_index() as usize,
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
                    .comparison(current_comparison)
                    [method];

                if pb_time.map_or(true, |t| time < t) {
                    SemanticColor::PersonalBest
                } else {
                    SemanticColor::BehindLosingTime
                }
            }
            _ => SemanticColor::NotRunning,
        };

        let visual_color = semantic_color.visualize(layout_settings);
        let (top_color, bottom_color) = top_and_bottom_color(visual_color);

        State {
            time: formatter::Time::with_digits_format(self.settings.digits_format)
                .format(time)
                .to_string(),
            fraction: formatter::Fraction::with_accuracy(self.settings.accuracy)
                .format(time)
                .to_string(),
            semantic_color,
            top_color,
            bottom_color,
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Timing Method".into(), self.settings.timing_method.into()),
            Field::new("Digits Format".into(), self.settings.digits_format.into()),
            Field::new("Accuracy".into(), self.settings.accuracy.into()),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.timing_method = value.into(),
            1 => self.settings.digits_format = value.into(),
            2 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}

pub fn top_and_bottom_color(color: Color) -> (Color, Color) {
    let hsv: Hsva = color.rgba.color.into();
    let top_color = Rgba::from(Hsva::new(
        hsv.hue,
        0.5 * hsv.saturation,
        (1.5 * hsv.value + 0.1).min(1.0),
        hsv.alpha,
    )).into();
    let bottom_color = Rgba::from(Hsva::new(
        hsv.hue,
        hsv.saturation,
        0.8 * hsv.value,
        hsv.alpha,
    )).into();
    (top_color, bottom_color)
}
