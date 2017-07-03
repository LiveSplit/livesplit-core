use {analysis, Timer, TimerPhase, Color};
use time_formatter::{Delta, TimeFormatter};
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use layout::editor::settings_description::{SettingsDescription, Value};

#[derive(Default, Clone)]
pub struct Component;

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

    pub fn name(&self) -> Cow<str> {
        "Previous Segment".into()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let mut time_change = None;
        let mut live_segment = false;
        let mut name = "Previous Segment";

        let phase = timer.current_phase();
        let method = timer.current_timing_method();
        let split_index = timer.current_split_index() as usize;
        let comparison = timer.current_comparison();
        let color = if phase != TimerPhase::NotRunning {
            if (phase == TimerPhase::Running || phase == TimerPhase::Paused) &&
                analysis::check_live_delta(timer, false, comparison, method).is_some()
            {
                live_segment = true;
            }

            if live_segment {
                time_change = analysis::live_segment_delta(timer, split_index, comparison, method);
                name = "Live Segment";
            } else if let Some(prev_split_index) = split_index.checked_sub(1) {
                time_change =
                    analysis::previous_segment_delta(timer, prev_split_index, comparison, method);
            }

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

        State {
            text: name.into(),
            time: Delta::new().format(time_change).to_string(),
            color: color,
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::default()
    }

    pub fn set_value(&mut self, _index: usize, _value: Value) {}
}
