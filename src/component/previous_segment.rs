use {analysis, Timer, TimerPhase, Color, comparison};
use time_formatter::{Delta, PossibleTimeSave, TimeFormatter, Accuracy};
use serde_json::{to_writer, Result};
use std::io::Write;
use std::fmt::Write as FmtWrite;
use std::borrow::Cow;
use layout::editor::settings_description::{SettingsDescription, Value, Field};

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub comparison_override: Option<String>,
    pub drop_decimals: bool,
    pub accuracy: Accuracy,
    pub show_possible_time_save: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            comparison_override: None,
            drop_decimals: true,
            accuracy: Accuracy::Tenths,
            show_possible_time_save: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub text: String,
    pub time: String,
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
        self.text(
            false,
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_ref),
        )
    }

    fn text(&self, live: bool, comparison: Option<&str>) -> Cow<str> {
        let text = if live {
            "Live Segment"
        } else {
            "Previous Segment"
        };
        let mut text = Cow::from(text);
        if let Some(comparison) = comparison {
            write!(text.to_mut(), " ({})", comparison::shorten(comparison)).unwrap();
        }
        text
    }

    pub fn state(&self, timer: &Timer) -> State {
        let mut time_change = None;
        let mut previous_possible = None;
        let mut live_segment = false;
        let resolved_comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(resolved_comparison, timer);

        let phase = timer.current_phase();
        let method = timer.current_timing_method();
        let split_index = timer.current_split_index() as usize;
        let color = if phase != TimerPhase::NotRunning {
            if (phase == TimerPhase::Running || phase == TimerPhase::Paused) &&
                analysis::check_live_delta(timer, false, comparison, method).is_some()
            {
                live_segment = true;
            }

            if live_segment {
                time_change = analysis::live_segment_delta(timer, split_index, comparison, method);
                if self.settings.show_possible_time_save {
                    previous_possible = analysis::possible_time_save::calculate(
                        timer,
                        split_index,
                        comparison,
                        false,
                    );
                }
            } else if let Some(prev_split_index) = split_index.checked_sub(1) {
                time_change =
                    analysis::previous_segment_delta(timer, prev_split_index, comparison, method);
                if self.settings.show_possible_time_save {
                    previous_possible = analysis::possible_time_save::calculate(
                        timer,
                        prev_split_index,
                        comparison,
                        false,
                    );
                }
            };

            if let Some(time_change) = time_change {
                if live_segment {
                    analysis::split_color(
                        timer,
                        time_change.into(),
                        split_index,
                        false,
                        false,
                        comparison,
                        method,
                    )
                } else if let Some(prev_split_index) = split_index.checked_sub(1) {
                    analysis::split_color(
                        timer,
                        time_change.into(),
                        prev_split_index,
                        false,
                        true,
                        comparison,
                        method,
                    )
                } else {
                    Color::Default
                }
            } else if let Some(prev_split_index) = split_index.checked_sub(1) {
                analysis::split_color(
                    timer,
                    None,
                    prev_split_index,
                    true,
                    true,
                    comparison,
                    method,
                )
            } else {
                Color::Default
            }
        } else {
            Color::Default
        };

        let text = self.text(live_segment, resolved_comparison);
        let mut time = Delta::custom(self.settings.drop_decimals, self.settings.accuracy)
            .format(time_change)
            .to_string();

        if self.settings.show_possible_time_save {
            write!(
                time,
                " / {}",
                PossibleTimeSave::with_accuracy(self.settings.accuracy).format(previous_possible)
            ).unwrap();
        }

        State {
            text: text.into_owned(),
            time: time,
            color: color,
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Comparison".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new("Drop Decimals".into(), self.settings.drop_decimals.into()),
            Field::new("Accuracy".into(), self.settings.accuracy.into()),
            Field::new(
                "Show Possible Time Save".into(),
                self.settings.show_possible_time_save.into(),
            ),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.comparison_override = value.into(),
            1 => self.settings.drop_decimals = value.into(),
            2 => self.settings.accuracy = value.into(),
            3 => self.settings.show_possible_time_save = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
