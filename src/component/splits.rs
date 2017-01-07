use std::io::Write;
use serde_json::{to_writer, Result};
use {Timer, TimeSpan};
use state_helper::split_color;
use time_formatter::{Delta, Regular, TimeFormatter};
use time_formatter::none_wrapper::EmptyWrapper;

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
    pub color: String,
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

        State {
            splits: timer.run()
                .segments()
                .iter()
                .zip(self.icon_ids.iter_mut())
                .enumerate()
                .map(|(i, (segment, icon_id))| {
                    let split = segment.split_time()[method];
                    let pb = segment.personal_best_split_time()[method];

                    let (time, delta) = if current_split > i as isize {
                        (split, TimeSpan::option_op(split, pb, |split, pb| split - pb))
                    } else {
                        (pb, None)
                    };

                    SplitState {
                        icon_change: segment.icon().check_for_change(icon_id).map(str::to_owned),
                        name: segment.name().to_string(),
                        delta: EmptyWrapper::new(Delta::with_decimal_dropping())
                            .format(delta)
                            .to_string(),
                        time: Regular::new().format(time).to_string(),
                        color: format!("{:?}",
                                       split_color(timer,
                                                   delta,
                                                   i,
                                                   true,
                                                   true,
                                                   "Personal Best",
                                                   method)),
                        is_current_split: i as isize == current_split,
                    }
                })
                .collect(),
        }
    }
}
