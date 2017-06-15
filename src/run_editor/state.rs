use super::{RunEditor, SegmentRow, TimingMethod};
use comparison::personal_best;
use time_formatter::{Accuracy, TimeFormatter, Short};
use time_formatter::none_wrapper::EmptyWrapper;
use serde_json::{to_writer, Result as JsonResult};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub icon_change: Option<String>,
    pub game: String,
    pub category: String,
    pub offset: String,
    pub attempts: u32,
    pub timing_method: TimingMethod,
    pub segments: Vec<Segment>,
    pub comparison_names: Vec<String>,
    pub buttons: Buttons,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buttons {
    pub can_remove: bool,
    pub can_move_up: bool,
    pub can_move_down: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub icon_change: Option<String>,
    pub name: String,
    pub split_time: String,
    pub segment_time: String,
    pub best_segment_time: String,
    pub comparison_times: Vec<String>,
    pub selected: SelectionState,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SelectionState {
    NotSelected,
    Selected,
    CurrentRow,
}

impl State {
    pub fn write_json<W>(&self, writer: W) -> JsonResult<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl RunEditor {
    pub fn state(&mut self) -> State {
        let formatter = EmptyWrapper::new(Short::with_accuracy(Accuracy::Hundredths));

        let icon_change = self.run
            .game_icon()
            .check_for_change(&mut self.game_icon_id)
            .map(str::to_owned);
        let game = self.game_name().to_string();
        let category = self.category_name().to_string();
        let offset = formatter.format(self.offset()).to_string();
        let attempts = self.attempt_count();
        let timing_method = self.selected_timing_method();
        let comparison_names = self.custom_comparisons()
            .iter()
            .cloned()
            .filter(|n| n != personal_best::NAME)
            .collect::<Vec<_>>();

        let buttons = Buttons {
            can_remove: self.can_remove_segments(),
            can_move_up: self.can_move_segments_up(),
            can_move_down: self.can_move_segments_down(),
        };
        let mut segments = Vec::with_capacity(self.run.len());

        self.segment_icon_ids.resize(self.run.len(), 0);
        for segment_index in 0..self.run.len() {
            let (name, split_time, segment_time, best_segment_time, comparison_times);
            {
                let row = SegmentRow::new(segment_index, self);
                name = row.name().to_string();
                split_time = formatter.format(row.split_time()).to_string();
                segment_time = formatter.format(row.segment_time()).to_string();
                best_segment_time = formatter.format(row.best_segment_time()).to_string();
                comparison_times = comparison_names
                    .iter()
                    .map(|c| formatter.format(row.comparison_time(c)).to_string())
                    .collect();
            }

            let icon_change = self.run
                .segment(segment_index)
                .icon()
                .check_for_change(&mut self.segment_icon_ids[segment_index])
                .map(str::to_owned);

            let selected = if self.selected_segment_index() == segment_index {
                SelectionState::CurrentRow
            } else if self.selected_segments.contains(&segment_index) {
                SelectionState::Selected
            } else {
                SelectionState::NotSelected
            };

            segments.push(Segment {
                icon_change: icon_change,
                name: name,
                split_time: split_time,
                segment_time: segment_time,
                best_segment_time: best_segment_time,
                comparison_times: comparison_times,
                selected: selected,
            });
        }

        State {
            icon_change: icon_change,
            game: game,
            category: category,
            offset: offset,
            attempts: attempts,
            timing_method: timing_method,
            segments: segments,
            comparison_names: comparison_names,
            buttons: buttons,
        }
    }
}
