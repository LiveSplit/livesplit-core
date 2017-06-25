use std::cmp::{min, max};
use std::io::Write;
use serde_json::{to_writer, Result};
use {Timer, TimeSpan, analysis};
use analysis::split_color;
use time_formatter::{Delta, Regular, TimeFormatter};
use time_formatter::none_wrapper::{EmptyWrapper, DashWrapper};
use Color;
use std::borrow::Cow;

#[derive(Default, Clone)]
pub struct Component {
    icon_ids: Vec<usize>,
    settings: Settings,
    scroll_offset: isize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub visual_split_count: usize,
    pub split_preview_count: usize,
    pub always_show_last_split: bool,
    pub separator_last_split: bool,
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
    pub show_final_separator: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            visual_split_count: 16,
            split_preview_count: 1,
            always_show_last_split: true,
            separator_last_split: true,
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

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    pub fn remount(&mut self) {
        self.icon_ids.clear();
    }

    pub fn name(&self) -> Cow<str> {
        "Splits".into()
    }

    pub fn state(&mut self, timer: &Timer) -> State {
        self.icon_ids.resize(timer.run().len(), 0);

        let current_split = timer.current_split_index();
        let method = timer.current_timing_method();
        let comparison = timer.current_comparison();

        let always_show_last_split = if self.settings.always_show_last_split {
            0
        } else {
            1
        };
        let skip_count = min(
            max(
                0,
                current_split as isize -
                    (self.settings.visual_split_count as isize - 2 -
                         self.settings.split_preview_count as isize +
                         always_show_last_split),
            ),
            timer.run().len() as isize - self.settings.visual_split_count as isize,
        );
        self.scroll_offset = min(
            max(self.scroll_offset, -skip_count),
            timer.run().len() as isize - skip_count -
                self.settings.visual_split_count as isize,
        );
        let skip_count = max(0, skip_count + self.scroll_offset) as usize;
        let take_count = self.settings.visual_split_count + always_show_last_split as usize - 1;
        let always_show_last_split = self.settings.always_show_last_split;

        let show_final_separator = self.settings.separator_last_split && always_show_last_split &&
            skip_count + take_count + 1 < timer.run().len();

        State {
            splits: timer
                .run()
                .segments()
                .iter()
                .enumerate()
                .skip(skip_count)
                .zip(self.icon_ids.iter_mut())
                .filter(|&((i, _), _)| {
                    i - skip_count < take_count ||
                        (always_show_last_split && i + 1 == timer.run().len())
                })
                .map(|((i, segment), icon_id)| {
                    let split = segment.split_time()[method];
                    let comparison_time = segment.comparison(comparison)[method];

                    let (time, delta, color) = if current_split > i as isize {
                        let delta = TimeSpan::option_sub(split, comparison_time);
                        (
                            split,
                            delta,
                            split_color(timer, delta, i, true, true, comparison, method),
                        )
                    } else if current_split == i as isize {
                        (
                            comparison_time,
                            analysis::check_live_delta(timer, true, comparison, method),
                            Color::Default,
                        )
                    } else {
                        (comparison_time, None, Color::Default)
                    };

                    let delta = if current_split > i as isize {
                        DashWrapper::new(Delta::with_decimal_dropping())
                            .format(delta)
                            .to_string()
                    } else {
                        EmptyWrapper::new(Delta::with_decimal_dropping())
                            .format(delta)
                            .to_string()
                    };

                    SplitState {
                        icon_change: segment.icon().check_for_change(icon_id).map(str::to_owned),
                        name: segment.name().to_string(),
                        delta: delta,
                        time: Regular::new().format(time).to_string(),
                        color: color,
                        is_current_split: i as isize == current_split,
                    }
                })
                .collect(),
            show_final_separator: show_final_separator,
        }
    }
}
