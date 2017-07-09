use {Color, Timer, TimerPhase, TimingMethod, TimeSpan};
use super::timer;
use time_formatter::{timer as formatter, TimeFormatter, Short, Accuracy, DigitsFormat};
use time_formatter::none_wrapper::DashWrapper;
use comparison::{self, best_segments, none};
use std::cmp::max;
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use layout::editor::settings_description::{SettingsDescription, Field, Value};

#[derive(Default, Clone)]
pub struct Component {
    timer: timer::Component,
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub comparison1: Option<String>,
    pub comparison2: Option<String>,
    pub hide_second_comparison: bool,
    pub timer: timer::Settings,
    pub segment_timer: timer::Settings,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub timer: timer::State,
    pub segment_timer: Option<timer::State>,
    pub comparison1: Option<ComparisonState>,
    pub comparison2: Option<ComparisonState>,
}

#[derive(Serialize, Deserialize)]
pub struct ComparisonState {
    pub name: String,
    pub time: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            comparison1: None,
            comparison2: Some(String::from(best_segments::NAME)),
            hide_second_comparison: false,
            timer: Default::default(),
            segment_timer: Default::default(),
        }
    }
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
        let timer = timer::Component::with_settings(settings.timer.clone());
        Self {
            timer,
            settings,
            ..Default::default()
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn set_settings(&mut self, settings: Settings) {
        self.settings = settings;
        *self.timer.settings_mut() = self.settings.timer.clone();
    }

    pub fn name(&self) -> Cow<str> {
        "Detailed Timer".into()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let current_phase = timer.current_phase();
        let timing_method = timer.current_timing_method();

        let last_split_index = if current_phase == TimerPhase::Ended {
            timer.run().len() - 1
        } else {
            max(0, timer.current_split_index()) as usize
        };

        let (comparison1, comparison2) = if current_phase != TimerPhase::NotRunning {
            let mut comparison1 = self.settings
                .comparison1
                .as_ref()
                .map(String::as_str)
                .unwrap_or_else(|| timer.current_comparison());

            let comparison2 = self.settings
                .comparison2
                .as_ref()
                .map(String::as_str)
                .unwrap_or_else(|| timer.current_comparison());

            let mut hide_comparison = self.settings.hide_second_comparison;

            if hide_comparison || !timer.run().comparisons().any(|c| c == comparison2) ||
                comparison2 == none::NAME
            {
                hide_comparison = true;
                if !timer.run().comparisons().any(|c| c == comparison1) ||
                    comparison1 == none::NAME
                {
                    comparison1 = timer.current_comparison();
                }
            } else if !timer.run().comparisons().any(|c| c == comparison1) ||
                       comparison1 == none::NAME
            {
                hide_comparison = true;
                comparison1 = comparison2;
            } else if comparison1 == comparison2 {
                hide_comparison = true;
            }

            let comparison1 = Some((
                comparison::shorten(comparison1),
                calculate_comparison_time(timer, timing_method, comparison1, last_split_index),
            ));

            let comparison2 = if !hide_comparison {
                Some((
                    comparison::shorten(comparison2),
                    calculate_comparison_time(timer, timing_method, comparison2, last_split_index),
                ))
            } else {
                None
            };

            (comparison1, comparison2)
        } else {
            Default::default()
        };

        let timer_state = self.timer.state(timer);
        let segment_time = calculate_segment_time(timer, timing_method, last_split_index);
        let segment_time_state = segment_time.map(|t| {
            timer::State {
                time: formatter::Time::with_digits_format(
                    self.settings.segment_timer.digits_format,
                ).format(t)
                    .to_string(),
                fraction: formatter::Fraction::with_accuracy(self.settings.segment_timer.accuracy)
                    .format(t)
                    .to_string(),
                color: Color::Default,
            }
        });

        let formatter = DashWrapper::new(Short::new());

        let comparison1 = comparison1.map(|(name, time)| {
            ComparisonState {
                name: name.to_string(),
                time: formatter.format(time).to_string(),
            }
        });

        let comparison2 = comparison2.map(|(name, time)| {
            ComparisonState {
                name: name.to_string(),
                time: formatter.format(time).to_string(),
            }
        });

        State {
            timer: timer_state,
            segment_timer: segment_time_state,
            comparison1,
            comparison2,
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Comparison 1".into(),
                self.settings.comparison1.clone().into(),
            ),
            Field::new(
                "Comparison 2".into(),
                self.settings.comparison2.clone().into(),
            ),
            Field::new(
                "Hide Second Comparison".into(),
                self.settings.hide_second_comparison.into(),
            ),
            Field::new(
                "Timer Digits Format".into(),
                self.settings.timer.digits_format.into(),
            ),
            Field::new(
                "Timer Accuracy".into(),
                self.settings.timer.accuracy.into(),
            ),
            Field::new(
                "Segment Timer Digits Format".into(),
                self.settings.segment_timer.digits_format.into(),
            ),
            Field::new(
                "Segment Timer Accuracy".into(),
                self.settings.segment_timer.accuracy.into(),
            ),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.comparison1 = value.into(),
            1 => self.settings.comparison2 = value.into(),
            2 => self.settings.hide_second_comparison = value.into(),
            3 => {
                let value: DigitsFormat = value.into();
                self.settings.timer.digits_format = value.clone();
                self.timer.settings_mut().digits_format = value;
            }
            4 => {
                let value: Accuracy = value.into();
                self.settings.timer.accuracy = value.clone();
                self.timer.settings_mut().accuracy = value;
            }
            5 => self.settings.segment_timer.digits_format = value.into(),
            6 => self.settings.segment_timer.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}

fn calculate_comparison_time(
    timer: &Timer,
    timing_method: TimingMethod,
    comparison: &str,
    last_split_index: usize,
) -> Option<TimeSpan> {
    if comparison == best_segments::NAME {
        timer.run().segment(last_split_index).best_segment_time()[timing_method]
    } else if last_split_index == 0 {
        timer
            .run()
            .segment(0)
            .comparison_timing_method(comparison, timing_method)
    } else if timer.current_split_index() > 0 {
        TimeSpan::option_sub(
            timer
                .run()
                .segment(last_split_index)
                .comparison_timing_method(comparison, timing_method),
            timer
                .run()
                .segment(last_split_index - 1)
                .comparison_timing_method(comparison, timing_method),
        )
    } else {
        None
    }
}

fn calculate_segment_time(
    timer: &Timer,
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
        TimeSpan::option_sub(timer.current_time()[timing_method], last_split)
    }
}
