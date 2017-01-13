use std::io::Write;
use serde_json::{to_writer, Result};
use {Timer, TimeSpan, state_helper};
use state_helper::split_color;
use time_formatter::{Delta, Regular, TimeFormatter};
use time_formatter::none_wrapper::EmptyWrapper;
use Color;

#[derive(Default)]
pub struct Component {
    icon_ids: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct SplitState {
    pub icon_change: Option<String>,
    pub name: String,
    pub delta: String,
    pub time: String,
    pub color: Color,
    pub is_current_split: bool,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub splits: Vec<SplitState>,
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

    pub fn state(&mut self, timer: &Timer) -> State {
        self.icon_ids.resize(timer.run().len(), 0);

        let current_split = timer.current_split_index();
        let method = timer.current_timing_method();
        let comparison = timer.current_comparison();

        State {
            splits: timer.run()
                .segments()
                .iter()
                .zip(self.icon_ids.iter_mut())
                .enumerate()
                .map(|(i, (segment, icon_id))| {
                    let split = segment.split_time()[method];
                    let comparison_time = segment.comparison(comparison)[method];

                    let (time, delta, color) = if current_split > i as isize {
                        let delta =
                            TimeSpan::option_op(split, comparison_time, |split, ct| split - ct);
                        (split, delta, split_color(timer, delta, i, true, true, comparison, method))
                    } else if current_split == i as isize {
                        (comparison_time,
                         state_helper::check_live_delta(timer, true, comparison, method),
                         Color::Default)
                    } else {
                        (comparison_time, None, Color::Default)
                    };

                    SplitState {
                        icon_change: segment.icon().check_for_change(icon_id).map(str::to_owned),
                        name: segment.name().to_string(),
                        delta: EmptyWrapper::new(Delta::with_decimal_dropping())
                            .format(delta)
                            .to_string(),
                        time: Regular::new().format(time).to_string(),
                        color: color,
                        is_current_split: i as isize == current_split,
                    }
                })
                .collect(),
        }
    }
}
