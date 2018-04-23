use super::{Editor, SegmentRow, TimingMethod};
use comparison::personal_best;
use serde_json::{to_writer, Result as JsonResult};
use std::io::Write;
use time::formatter::none_wrapper::EmptyWrapper;
use time::formatter::{Accuracy, Short, TimeFormatter};

/// Represents the current state of the Run Editor in order to visualize it
/// properly.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    /// The game's icon encoded as a Data URL. This value is only specified
    /// whenever the icon changes. The String itself may be empty. This
    /// indicates that there is no icon.
    pub icon_change: Option<String>,
    /// The name of the game the Run is for.
    pub game: String,
    /// The name of the category the Run is for.
    pub category: String,
    /// The timer offset specifies the time that the timer starts at when starting a
    /// new attempt.
    pub offset: String,
    /// The number of times this Run has been attempted by the runner. This
    /// is mostly just a visual number and has no effect on any history.
    pub attempts: u32,
    /// The timing method that is currently selected to be visualized and
    /// edited.
    pub timing_method: TimingMethod,
    /// The state of all the segments.
    pub segments: Vec<Segment>,
    /// The names of all the custom comparisons that exist for this Run.
    pub comparison_names: Vec<String>,
    /// Describes which actions are currently available.
    pub buttons: Buttons,
}

/// Describes which actions are currently available. Depending on how many
/// segments exist and which ones are selected, only some actions can be
/// executed successfully.
#[derive(Debug, Serialize, Deserialize)]
pub struct Buttons {
    /// Describes whether the currently selected segments can be removed. If all
    /// segments are selected, they can't be removed.
    pub can_remove: bool,
    /// Describes whether the currently selected segments can be moved up. If
    /// any one of the selected segments is the first segment, then they can't
    /// be moved.
    pub can_move_up: bool,
    /// Describes whether the currently selected segments can be moved down. If
    /// any one of the selected segments is the last segment, then they can't be
    /// moved.
    pub can_move_down: bool,
}

/// Describes the current state of a segment.
#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    /// The segment's icon encoded as a Data URL. This value is only specified
    /// whenever the icon changes. The String itself may be empty. This
    /// indicates that there is no icon.
    pub icon_change: Option<String>,
    /// The name of the segment.
    pub name: String,
    /// The segment's split time for the active timing method.
    pub split_time: String,
    /// The segment time for the active timing method.
    pub segment_time: String,
    /// The best segment time for the active timing method.
    pub best_segment_time: String,
    /// All of the times of the custom comparison for the active timing method.
    /// The order of these matches up with the order of the custom comparisons
    /// provided by the Run Editor's State object.
    pub comparison_times: Vec<String>,
    /// Describes the segment's selection state.
    pub selected: SelectionState,
}

/// Describes a segment's selection state.
#[derive(Debug, Serialize, Deserialize)]
pub enum SelectionState {
    /// The segment is not selected.
    NotSelected,
    /// The segment is selected.
    Selected,
    /// The segment is selected and active. There is only one active segment and
    /// it is the one that is being actively edited.
    Active,
}

impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> JsonResult<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Editor {
    /// Calculates the Run Editor's state in order to visualize it.
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

            let selected = if self.active_segment_index() == segment_index {
                SelectionState::Active
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
