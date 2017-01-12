use {Color, Timer, TimerPhase, TimeSpan};
use time_formatter::{timer as formatter, TimeFormatter};
use state_helper::split_color;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub time: String,
    pub fraction: String,
    pub color: Color,
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
        let method = timer.current_timing_method();
        let time = timer.current_time();
        let time = time[method].unwrap();
        let current_comparison = timer.current_comparison();

        let color = match timer.current_phase() {
            TimerPhase::Running if time >= TimeSpan::zero() => {
                let pb_split_time = timer.current_split()
                    .unwrap()
                    .comparison(current_comparison)
                    .and_then(|t| t[method]);
                if let Some(pb_split_time) = pb_split_time {
                    split_color(timer,
                                Some(time - pb_split_time),
                                timer.current_split_index() as usize,
                                true,
                                false,
                                current_comparison,
                                method)
                        .or(Color::AheadGainingTime)
                } else {
                    Color::AheadGainingTime
                }
            }
            TimerPhase::Paused => Color::Paused,
            TimerPhase::Ended => {
                let pb_time = timer.run()
                    .segments()
                    .last()
                    .unwrap()
                    .comparison(current_comparison)
                    .and_then(|t| t[method]);
                if pb_time.map_or(true, |t| time < t) {
                    Color::PersonalBest
                } else {
                    Color::BehindLosingTime
                }
            }
            _ => Color::NotRunning,
        };

        State {
            time: formatter::Time.format(time).to_string(),
            fraction: formatter::Fraction.format(time).to_string(),
            color: color,
        }
    }
}
