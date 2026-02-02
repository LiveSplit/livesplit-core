use super::{Editor, SegmentRow, TimingMethod};
use crate::{
    comparison::personal_best,
    localization::Lang,
    platform::prelude::*,
    run::RunMetadata,
    settings::{ImageCache, ImageId},
    timing::formatter::{Accuracy, SegmentTime, TimeFormatter, none_wrapper::EmptyWrapper},
};
use serde_derive::{Deserialize, Serialize};

/// Represents the current state of the Run Editor in order to visualize it
/// properly.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    /// The game icon of the run. The associated image can be looked up in the
    /// image cache. The image may be the empty image. This indicates that there
    /// is no icon.
    pub icon: ImageId,
    /// The name of the game the Run is for.
    pub game: String,
    /// The name of the category the Run is for.
    pub category: String,
    /// The timer offset specifies the time that the timer starts at when
    /// starting a new attempt.
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
    /// Additional metadata of this Run, like the platform and region of the
    /// game.
    pub metadata: RunMetadata,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Segment {
    /// The icon of the segment. The associated image can be looked up in the
    /// image cache. The image may be the empty image. This indicates that there
    /// is no icon.
    pub icon: ImageId,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SelectionState {
    /// The segment is not selected.
    NotSelected,
    /// The segment is selected.
    Selected,
    /// The segment is selected and active. There is only one active segment and
    /// it is the one that is being actively edited.
    Active,
}

impl SelectionState {
    /// Returns `true` if the segment is selected.
    pub const fn is_selected_or_active(&self) -> bool {
        matches!(self, Self::Selected | Self::Active)
    }
}

#[cfg(feature = "std")]
impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}

impl Editor {
    /// Calculates the Run Editor's state in order to visualize it. The
    /// [`ImageCache`] is updated with all the images that are part of the
    /// state. The images are marked as visited in the [`ImageCache`]. You still
    /// need to manually run [`ImageCache::collect`] to ensure unused images are
    /// removed from the cache.
    pub fn state(&self, image_cache: &mut ImageCache, lang: Lang) -> State {
        let formatter = EmptyWrapper::new(SegmentTime::with_accuracy(Accuracy::Hundredths));

        let icon = self.run.game_icon();
        let icon = *image_cache.cache(icon.id(), || icon.clone()).id();

        let game = self.game_name().to_string();
        let category = self.category_name().to_string();
        let offset = formatter.format(self.offset(), lang).to_string();
        let attempts = self.attempt_count();
        let timing_method = self.selected_timing_method();
        let comparison_names = self
            .custom_comparisons()
            .iter()
            .filter(|&n| n != personal_best::NAME)
            .cloned()
            .collect::<Vec<_>>();

        let buttons = Buttons {
            can_remove: self.can_remove_segments(),
            can_move_up: self.can_move_segments_up(),
            can_move_down: self.can_move_segments_down(),
        };
        let mut segments = Vec::with_capacity(self.run.len());

        for segment_index in 0..self.run.len() {
            let (name, split_time, segment_time, best_segment_time, comparison_times);
            {
                let row = SegmentRow::new(segment_index, self);
                name = row.name().to_string();
                split_time = formatter.format(row.split_time(), lang).to_string();
                segment_time = formatter.format(row.segment_time(), lang).to_string();
                best_segment_time = formatter.format(row.best_segment_time(), lang).to_string();
                comparison_times = comparison_names
                    .iter()
                    .map(|c| formatter.format(row.comparison_time(c), lang).to_string())
                    .collect();
            }

            let icon = self.run.segment(segment_index).icon();
            let icon = *image_cache.cache(icon.id(), || icon.clone()).id();

            let selected = if self.active_segment_index() == segment_index {
                SelectionState::Active
            } else if self.selected_segments.contains(&segment_index) {
                SelectionState::Selected
            } else {
                SelectionState::NotSelected
            };

            segments.push(Segment {
                icon,
                name,
                split_time,
                segment_time,
                best_segment_time,
                comparison_times,
                selected,
            });
        }

        State {
            icon,
            game,
            category,
            offset,
            attempts,
            timing_method,
            segments,
            comparison_names,
            buttons,
            metadata: self.run.metadata().clone(),
        }
    }
}
