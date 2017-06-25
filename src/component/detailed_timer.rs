use {Color, Timer, TimerPhase, TimingMethod, TimeSpan};
use super::timer;
use time_formatter::{timer as formatter, TimeFormatter, Short};
use time_formatter::none_wrapper::DashWrapper;
use comparison::{best_segments, none};
use std::cmp::max;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component {
    timer: timer::Component,
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub comparison1: Option<String>,
    pub comparison2: Option<String>,
    pub hide_comparison: bool,
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
            hide_comparison: false,
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

            let mut hide_comparison = self.settings.hide_comparison;

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
                comparison1,
                calculate_comparison_time(
                    timer,
                    timing_method,
                    comparison1,
                    last_split_index,
                ),
            ));

            let comparison2 = if !hide_comparison {
                Some((
                    comparison2,
                    calculate_comparison_time(
                        timer,
                        timing_method,
                        comparison2,
                        last_split_index,
                    ),
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
                time: formatter::Time.format(t).to_string(),
                fraction: formatter::Fraction.format(t).to_string(),
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
        timer.run().segment(0).comparison_timing_method(
            comparison,
            timing_method,
        )
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
