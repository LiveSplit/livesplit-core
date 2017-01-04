use {Timer, TimeSpan};
use time_formatter::{Delta, Regular, TimeFormatter};
use time_formatter::none_wrapper::{DashWrapper, EmptyWrapper};
use serde_json::{to_writer, Result};
use std::io::Write;

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

        State {
            splits: timer.run()
                .segments()
                .iter()
                .zip(self.icon_ids.iter_mut())
                .enumerate()
                .map(|(i, (segment, icon_id))| {
                    let split = segment.split_time().real_time;
                    let pb = segment.personal_best_split_time().real_time;

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
                        time: DashWrapper::new(Regular::new()).format(time).to_string(),
                        is_current_split: i as isize == current_split,
                    }
                })
                .collect(),
        }
    }
}
