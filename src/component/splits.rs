use {Timer, TimeSpan};
use time_formatter::{Delta, Regular, TimeFormatter};
use time_formatter::none_wrapper::{DashWrapper, EmptyWrapper};
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct SplitState {
    pub icon: String,
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

    pub fn state(&self, timer: &Timer) -> State {
        let current_split = timer.current_split_index();

        State {
            splits: timer.run()
                .segments()
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let split = s.split_time().real_time;
                    let pb = s.personal_best_split_time().real_time;

                    let (time, delta) = if current_split > i as isize {
                        (split, TimeSpan::option_op(split, pb, |split, pb| split - pb))
                    } else {
                        (pb, None)
                    };

                    SplitState {
                        icon: s.icon().to_string(),
                        name: s.name().to_string(),
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
