use {state_helper, Timer, TimerPhase};
use time_formatter::{Delta, TimeFormatter};
use run::PERSONAL_BEST_COMPARISON_NAME;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub text: String,
    pub time: String,
}

impl State {
    pub fn write_json<W>(&self, mut writer: W) -> Result<()>
        where W: Write
    {
        to_writer(&mut writer, self)
    }
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let mut time_change = None;
        let mut live_segment = false;
        let mut name = "Previous Segment";

        let phase = timer.current_phase();
        let method = timer.current_timing_method();
        let split_index = timer.current_split_index() as usize;
        let comparison = PERSONAL_BEST_COMPARISON_NAME; // TODO Make this the current comparison

        if phase != TimerPhase::NotRunning {
            if (phase == TimerPhase::Running || phase == TimerPhase::Paused) &&
               state_helper::check_live_delta(timer, false, comparison, method).is_some() {
                live_segment = true;
            }
            if live_segment {
                time_change =
                    state_helper::live_segment_delta(timer, split_index, comparison, method);
                name = "Live Segment";
            } else if split_index > 0 {
                time_change = state_helper::previous_segment_delta(timer,
                                                                   split_index - 1,
                                                                   comparison,
                                                                   method);
            }
        }
        State {
            text: name.into(),
            time: Delta::new().format(time_change).to_string(),
        }
    }
}
