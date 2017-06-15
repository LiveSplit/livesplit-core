use {analysis, Timer, TimeSpan, TimerPhase};
use time_formatter::{PossibleTimeSave, TimeFormatter};
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(new, Default)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub text: String,
    pub time: String,
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
    pub fn state(&self, timer: &Timer) -> State {
        let live = false;

        let segment_index = timer.current_split_index();
        let current_phase = timer.current_phase();

        let time = if current_phase == TimerPhase::Running || current_phase == TimerPhase::Paused {
            get_possible_time_save(
                timer,
                segment_index as usize,
                timer.current_comparison(),
                live,
            )
        } else {
            None
        };

        State {
            text: "Possible Time Save".to_string(),
            time: PossibleTimeSave::new().format(time).to_string(),
        }
    }
}

pub fn get_possible_time_save(
    timer: &Timer,
    segment_index: usize,
    comparison: &str,
    live: bool,
) -> Option<TimeSpan> {
    let segments = timer.run().segments();
    let method = timer.current_timing_method();
    let mut prev_time = TimeSpan::zero();
    let segment = timer.run().segment(segment_index);
    let mut best_segments = segment.best_segment_time()[method];

    for segment in segments[..segment_index].iter().rev() {
        if let Some(ref mut best_segments) = best_segments {
            if let Some(split_time) = segment.comparison(comparison)[method] {
                prev_time = split_time;
                break;
            } else if let Some(best_segment) = segment.best_segment_time()[method] {
                *best_segments += best_segment;
            }
        } else {
            break;
        }
    }

    let mut time = TimeSpan::option_op(
        segment.comparison(comparison)[method],
        best_segments,
        |c, b| c - b - prev_time,
    );

    if live && segment_index == timer.current_split_index() as usize {
        let segment_delta = analysis::live_segment_delta(timer, segment_index, comparison, method);
        if let (Some(segment_delta), Some(time)) = (segment_delta, time.as_mut()) {
            let segment_delta = TimeSpan::zero() - segment_delta;
            if segment_delta < *time {
                *time = segment_delta;
            }
        }
    }

    time.map(|t| if t < TimeSpan::zero() {
        TimeSpan::zero()
    } else {
        t
    })
}
