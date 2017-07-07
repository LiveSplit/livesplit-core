use {Color, Timer, TimerPhase, TimeSpan};
use time_formatter::{timer as formatter, TimeFormatter, Accuracy, DigitsFormat};
use analysis::split_color;
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use layout::editor::settings_description::{SettingsDescription, Value, Field};

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub digits_format: DigitsFormat,
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            digits_format: DigitsFormat::SingleDigitSeconds,
            accuracy: Accuracy::Hundredths,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub time: String,
    pub fraction: String,
    pub color: Color,
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

    pub fn state(&self, timer: &Timer) -> State {
        let method = timer.current_timing_method();
        let time = timer.current_time();
        let time = time[method].or(time.real_time).unwrap_or_default();
        let current_comparison = timer.current_comparison();

        let color = match timer.current_phase() {
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
                    ).or(Color::AheadGainingTime)
                } else {
                    Color::AheadGainingTime
                }
            }
            TimerPhase::Paused => Color::Paused,
            TimerPhase::Ended => {
                let pb_time = timer
                    .run()
                    .segments()
                    .last()
                    .unwrap()
                    .comparison(current_comparison)
                    [method];
                if pb_time.map_or(true, |t| time < t) {
                    Color::PersonalBest
                } else {
                    Color::BehindLosingTime
                }
            }
            _ => Color::NotRunning,
        };

        State {
            time: formatter::Time::with_digits_format(self.settings.digits_format)
                .format(time)
                .to_string(),
            fraction: formatter::Fraction::with_accuracy(self.settings.accuracy)
                .format(time)
                .to_string(),
            color: color,
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Digits Format".into(),
                self.settings.digits_format.into(),
            ),
            Field::new(
                "Accuracy".into(),
                self.settings.accuracy.into(),
            ),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.digits_format = value.into(),
            1 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
