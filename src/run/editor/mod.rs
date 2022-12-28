//! The editor module provides an editor for [`Run`] objects. The editor ensures
//! that all the different invariants of the [`Run`] objects are upheld no matter
//! what kind of operations are being applied to the [`Run`]. It provides the
//! current state of the editor as state objects that can be visualized by any
//! kind of User Interface.

use super::{ComparisonError, ComparisonResult};
use crate::{
    comparison,
    platform::prelude::*,
    settings::{CachedImageId, Image},
    timing::ParseError as ParseTimeSpanError,
    util::PopulateString,
    Run, Segment, Time, TimeSpan, TimingMethod,
};
use core::{mem::swap, num::ParseIntError};
use snafu::{OptionExt, ResultExt};

pub mod cleaning;
mod fuzzy_list;
mod segment_row;
mod state;
#[cfg(test)]
mod tests;

pub use self::{
    cleaning::SumOfBestCleaner,
    fuzzy_list::FuzzyList,
    segment_row::SegmentRow,
    state::{Buttons as ButtonsState, Segment as SegmentState, SelectionState, State},
};

/// Describes an Error that occurred while parsing a time.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum ParseError {
    /// Couldn't parse the time.
    ParseTime {
        /// The underlying error.
        source: ParseTimeSpanError,
    },
    /// Negative times are not allowed here.
    NegativeTimeNotAllowed,
    /// Empty times are not allowed here.
    EmptyTimeNotAllowed,
}

/// Describes an Error that occurred while opening the Run Editor.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum OpenError {
    /// The Run Editor couldn't be opened because an empty run with no
    /// segments was provided.
    EmptyRun,
}

/// Error type for a failed Rename.
#[derive(PartialEq, Eq, Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum RenameError {
    /// Old Comparison was not found during rename.
    OldNameNotFound,
    /// Name was invalid.
    InvalidName {
        /// The underlying error.
        source: ComparisonError,
    },
}

/// The Run Editor allows modifying Runs while ensuring that all the different
/// invariants of the Run objects are upheld no matter what kind of operations
/// are being applied to the Run. It provides the current state of the editor as
/// state objects that can be visualized by any kind of User Interface.
pub struct Editor {
    run: Run,
    selected_method: TimingMethod,
    selected_segments: Vec<usize>,
    previous_personal_best_time: Time,
    game_icon_id: CachedImageId,
    segment_icon_ids: Vec<CachedImageId>,
    segment_times: Vec<Option<TimeSpan>>,
}

impl Editor {
    /// Creates a new Run Editor that modifies the Run provided. Creation of the
    /// Run Editor fails when a Run with no segments is provided.
    pub fn new(mut run: Run) -> Result<Self, OpenError> {
        run.fix_splits();
        let len = run.len();
        if len == 0 {
            return Err(OpenError::EmptyRun);
        }

        let personal_best_time = run.segments().last().unwrap().personal_best_split_time();

        let mut editor = Self {
            run,
            selected_method: TimingMethod::RealTime,
            selected_segments: vec![0],
            previous_personal_best_time: personal_best_time,
            game_icon_id: CachedImageId::default(),
            segment_icon_ids: Vec::with_capacity(len),
            segment_times: Vec::with_capacity(len),
        };

        editor.update_segment_list();

        Ok(editor)
    }

    /// Closes the Run Editor and gives back access to the modified Run object.
    /// In case you want to implement a Cancel Button, just drop the Run object
    /// you get here.
    #[allow(clippy::missing_const_for_fn)] // FIXME: Drop is unsupported.
    pub fn close(self) -> Run {
        self.run
    }

    /// Accesses the Run being edited by the Run Editor.
    #[inline]
    pub const fn run(&self) -> &Run {
        &self.run
    }

    /// Accesses the timing method that is currently selected for being
    /// modified.
    pub const fn selected_timing_method(&self) -> TimingMethod {
        self.selected_method
    }

    /// Selects a different timing method for being modified.
    pub fn select_timing_method(&mut self, method: TimingMethod) {
        self.selected_method = method;
        self.update_segment_list();
    }

    fn active_segment_index(&self) -> usize {
        *self.selected_segments.last().unwrap()
    }

    /// Grants mutable access to the actively selected segment row. You can
    /// select multiple segment rows at the same time, but only the one that is
    /// most recently selected is the active segment.
    pub fn active_segment(&mut self) -> SegmentRow<'_> {
        SegmentRow::new(self.active_segment_index(), self)
    }

    /// Unselects the segment with the given index. If it's not selected or the
    /// index is out of bounds, nothing happens. The segment is not unselected,
    /// when it is the only segment that is selected. If the active segment is
    /// unselected, the most recently selected segment remaining becomes the
    /// active segment.
    pub fn unselect(&mut self, index: usize) {
        self.selected_segments.retain(|&i| i != index);
        if self.selected_segments.is_empty() {
            self.selected_segments.push(index);
        }
    }

    /// In addition to the segments that are already selected, the segment with
    /// the given index is being selected. The segment chosen also becomes the
    /// active segment.
    ///
    /// # Panics
    ///
    /// This panics if the index of the segment provided is out of bounds.
    pub fn select_additionally(&mut self, index: usize) {
        if index >= self.run.len() {
            panic!("Index out of bounds for segment selection.");
        }
        self.selected_segments.retain(|&i| i != index);
        self.selected_segments.push(index);
    }

    /// Select all segments from the currently active segment to the segment at
    /// the index provided. The segment at the index provided becomes the new
    /// active segment.
    ///
    /// # Panics
    ///
    /// This panics if the index of the segment provided is out of bounds.
    pub fn select_range(&mut self, index: usize) {
        let active = self.active_segment_index();
        let range = if index < active {
            index + 1..active
        } else {
            active + 1..index
        };
        for i in range {
            if !self.selected_segments.contains(&i) {
                self.selected_segments.push(i);
            }
        }
        self.select_additionally(index);
    }

    /// Selects the segment with the given index. All other segments are
    /// unselected. The segment chosen also becomes the active segment.
    ///
    /// # Panics
    ///
    /// This panics if the index of the segment provided is out of bounds.
    pub fn select_only(&mut self, index: usize) {
        if index >= self.run.len() {
            panic!("Index out of bounds for segment selection.");
        }
        self.selected_segments.clear();
        self.selected_segments.push(index);
    }

    fn raise_run_edited(&mut self) {
        self.run.mark_as_modified();
    }

    /// Accesses the name of the game.
    pub fn game_name(&self) -> &str {
        self.run.game_name()
    }

    /// Sets the name of the game.
    pub fn set_game_name<S>(&mut self, name: S)
    where
        S: PopulateString,
    {
        self.run.set_game_name(name);
        self.raise_run_edited();
        self.run.clear_run_id();
    }

    /// Accesses the name of the category.
    pub fn category_name(&self) -> &str {
        self.run.category_name()
    }

    /// Sets the name of the category.
    pub fn set_category_name<S>(&mut self, name: S)
    where
        S: PopulateString,
    {
        self.run.set_category_name(name);
        self.raise_run_edited();
        self.run.clear_run_id();
    }

    /// Accesses the timer offset. The timer offset specifies the time, the
    /// timer starts at when starting a new attempt.
    pub const fn offset(&self) -> TimeSpan {
        self.run.offset()
    }

    /// Sets the timer offset. The timer offset specifies the time, the timer
    /// starts at when starting a new attempt.
    pub fn set_offset(&mut self, offset: TimeSpan) {
        self.run.set_offset(offset);
        self.raise_run_edited();
    }

    /// Parses and sets the timer offset from the string provided. The timer
    /// offset specifies the time, the timer starts at when starting a new
    /// attempt.
    pub fn parse_and_set_offset(&mut self, offset: &str) -> Result<(), ParseError> {
        self.set_offset(offset.parse().context(ParseTime)?);
        Ok(())
    }

    /// Accesses the attempt count.
    pub const fn attempt_count(&self) -> u32 {
        self.run.attempt_count()
    }

    /// Sets the attempt count. Changing this has no affect on the attempt
    /// history or the segment history. This number is mostly just a visual
    /// number for the runner.
    pub fn set_attempt_count(&mut self, attempts: u32) {
        self.run.set_attempt_count(attempts);
        self.raise_run_edited();
    }

    /// Parses and sets the attempt count from the string provided. Changing
    /// this has no affect on the attempt history or the segment history. This
    /// number is mostly just a visual number for the runner.
    pub fn parse_and_set_attempt_count(&mut self, attempts: &str) -> Result<(), ParseIntError> {
        self.set_attempt_count(attempts.parse()?);
        Ok(())
    }

    /// Accesses the game's icon.
    pub const fn game_icon(&self) -> &Image {
        self.run.game_icon()
    }

    /// Sets the game's icon.
    pub fn set_game_icon<D: Into<Image>>(&mut self, image: D) {
        self.run.set_game_icon(image);
        self.raise_run_edited();
    }

    /// Removes the game's icon.
    pub fn remove_game_icon(&mut self) {
        self.run.set_game_icon([]);
        self.raise_run_edited();
    }

    /// Accesses all the custom comparisons that exist on the Run.
    pub fn custom_comparisons(&self) -> &[String] {
        self.run.custom_comparisons()
    }

    /// Sets the speedrun.com Run ID of the run. You need to ensure that the
    /// record on speedrun.com matches up with the Personal Best of this run.
    /// This may be empty if there's no association.
    pub fn set_run_id<S>(&mut self, id: S)
    where
        S: PopulateString,
    {
        self.run.metadata_mut().set_run_id(id);
        self.raise_run_edited();
    }

    /// Sets the name of the region this game is from. This may be empty if it's
    /// not specified.
    pub fn set_region_name<S>(&mut self, name: S)
    where
        S: PopulateString,
    {
        self.run.metadata_mut().set_region_name(name);
        self.metadata_modified();
    }

    /// Sets the name of the platform this game is run on. This may be empty if
    /// it's not specified.
    pub fn set_platform_name<S>(&mut self, name: S)
    where
        S: PopulateString,
    {
        self.run.metadata_mut().set_platform_name(name);
        self.metadata_modified();
    }

    /// Specifies whether this speedrun is done on an emulator. Keep in mind
    /// that `false` may also mean that this information is simply not known.
    pub fn set_emulator_usage(&mut self, uses_emulator: bool) {
        self.run.metadata_mut().set_emulator_usage(uses_emulator);
        self.metadata_modified();
    }

    /// Sets the speedrun.com variable with the name specified to the value
    /// specified. A variable is an arbitrary key value pair storing additional
    /// information about the category. An example of this may be whether
    /// Amiibos are used in this category. If the variable doesn't exist yet, it
    /// is being inserted.
    pub fn set_speedrun_com_variable<N, V>(&mut self, name: N, value: V)
    where
        N: PopulateString,
        V: PopulateString,
    {
        self.run
            .metadata_mut()
            .set_speedrun_com_variable(name, value);
        self.metadata_modified();
    }

    /// Removes the speedrun.com variable with the name specified.
    pub fn remove_speedrun_com_variable(&mut self, name: &str) {
        self.run.metadata_mut().remove_speedrun_com_variable(name);
        self.metadata_modified();
    }

    /// Adds a new permanent custom variable. If there's a temporary variable
    /// with the same name, it gets turned into a permanent variable and its
    /// value stays. If a permanent variable with the name already exists,
    /// nothing happens.
    pub fn add_custom_variable<N>(&mut self, name: N)
    where
        N: PopulateString,
    {
        self.run
            .metadata_mut()
            .custom_variable_mut(name)
            .permanent();
        self.raise_run_edited();
    }

    /// Sets the value of a custom variable with the name specified. If the
    /// custom variable does not exist, or is not a permanent variable, nothing
    /// happens.
    pub fn set_custom_variable<N, V>(&mut self, name: N, value: V)
    where
        N: PopulateString,
        V: PopulateString,
    {
        if self.run.metadata().custom_variable(name.as_str()).is_none() {
            return;
        }
        let variable = self.run.metadata_mut().custom_variable_mut(name);
        if variable.is_permanent {
            value.populate(&mut variable.value);
            self.raise_run_edited();
        }
    }

    /// Removes the custom variable with the name specified. If the custom
    /// variable does not exist, or is not a permanent variable, nothing
    /// happens.
    pub fn remove_custom_variable(&mut self, name: &str) {
        if let Some(variable) = self.run.metadata().custom_variable(name) {
            if variable.is_permanent {
                self.run.metadata_mut().remove_custom_variable(name);
                self.raise_run_edited();
            }
        }
    }

    /// Resets all the Metadata Information.
    pub fn clear_metadata(&mut self) {
        self.run.metadata_mut().clear();
        self.raise_run_edited();
    }

    fn metadata_modified(&mut self) {
        self.run.clear_run_id();
        self.raise_run_edited();
    }

    fn times_modified(&mut self) {
        let pb_split_time = self
            .run
            .segments()
            .last()
            .unwrap()
            .personal_best_split_time();

        if pb_split_time != self.previous_personal_best_time {
            self.run.clear_run_id();
            self.previous_personal_best_time = pb_split_time;
        }

        self.raise_run_edited();
    }

    fn fix(&mut self) {
        self.run.fix_splits();
        self.update_segment_list();
        self.raise_run_edited();
    }

    fn update_segment_list(&mut self) {
        let method = self.selected_method;
        let mut previous_time = Some(TimeSpan::zero());
        self.segment_times.clear();
        for segment in self.run.segments() {
            let split_time = segment.personal_best_split_time()[method];
            self.segment_times
                .push(catch! { split_time? - previous_time? });
            if split_time.is_some() {
                previous_time = split_time;
            }
        }
    }

    fn fix_splits_from_segments(&mut self) {
        let method = self.selected_method;
        let mut previous_time = Some(TimeSpan::zero());
        for (segment_time, segment) in self
            .segment_times
            .iter_mut()
            .zip(self.run.segments_mut().iter_mut())
        {
            {
                let time = segment.personal_best_split_time_mut();
                time[method] = catch! { previous_time? + (*segment_time)? };
            }
            if segment_time.is_some() {
                previous_time = segment.personal_best_split_time()[method];
            }
        }
    }

    /// Inserts a new empty segment above the active segment and adjusts the
    /// Run's history information accordingly. The newly created segment is then
    /// the only selected segment and also the active segment.
    pub fn insert_segment_above(&mut self) {
        let active_segment = self.active_segment_index();

        let mut segment = Segment::new("");
        self.run.import_best_segment(active_segment);

        let max_index = self.run.max_attempt_history_index().unwrap_or(0);
        let min_index = self.run.min_segment_history_index().unwrap();
        for x in min_index..=max_index {
            segment.segment_history_mut().insert(x, Default::default());
        }
        self.run.segments_mut().insert(active_segment, segment);

        self.select_only(active_segment);

        self.times_modified();
        self.fix();
    }

    /// Inserts a new empty segment below the active segment and adjusts the
    /// Run's history information accordingly. The newly created segment is then
    /// the only selected segment and also the active segment.
    pub fn insert_segment_below(&mut self) {
        let active_segment = self.active_segment_index();
        let next_segment = active_segment + 1;

        let mut segment = Segment::new("");
        if next_segment < self.run.len() {
            self.run.import_best_segment(next_segment);
        }

        let max_index = self.run.max_attempt_history_index().unwrap_or(0);
        let min_index = self.run.min_segment_history_index().unwrap();
        for x in min_index..=max_index {
            segment.segment_history_mut().insert(x, Default::default());
        }
        self.run.segments_mut().insert(next_segment, segment);

        self.select_only(next_segment);

        self.times_modified();
        self.fix();
    }

    fn fix_after_deletion(&mut self, index: usize) {
        self.fix_with_timing_method(index, TimingMethod::RealTime);
        self.fix_with_timing_method(index, TimingMethod::GameTime);
    }

    fn fix_with_timing_method(&mut self, index: usize, method: TimingMethod) {
        let current_index = index + 1;

        if current_index >= self.run.len() {
            return;
        }

        let max_index = self.run.max_attempt_history_index().unwrap_or(0);
        let min_index = self.run.min_segment_history_index().unwrap();
        for run_index in min_index..=max_index {
            // If a history element isn't there in the segment that's deleted
            // remove it from the next segment's history as well
            if let Some(segment_history_element) =
                self.run.segment(index).segment_history().get(run_index)
            {
                let current_segment = segment_history_element[method];
                if let Some(current_segment) = current_segment {
                    for current_index in current_index..self.run.len() {
                        // Add the removed segment's history times to the next
                        // non None times
                        if let Some(Some(segment)) = self
                            .run
                            .segment_mut(current_index)
                            .segment_history_mut()
                            .get_mut(run_index)
                            .map(|t| &mut t[method])
                        {
                            *segment += current_segment;
                            break;
                        }
                    }
                }
            } else {
                self.run
                    .segment_mut(current_index)
                    .segment_history_mut()
                    .remove(run_index);
            }
        }

        // Set the new Best Segment time to be the sum of the two Best Segments
        let min_best_segment = catch! {
            self.run.segment(index).best_segment_time()[method]?
                + self.run.segment(current_index).best_segment_time()[method]?
        };

        if let Some(mut min_best_segment) = min_best_segment {
            // Use any element in the history that has a lower time than this
            // sum
            for time in self
                .run
                .segment(current_index)
                .segment_history()
                .iter()
                .filter_map(|&(_, t)| t[method])
            {
                if time < min_best_segment {
                    min_best_segment = time;
                }
            }
            self.run.segment_mut(current_index).best_segment_time_mut()[method] =
                Some(min_best_segment);
        }
    }

    /// Checks if the currently selected segments can be removed. If all
    /// segments are selected, they can't be removed.
    pub fn can_remove_segments(&self) -> bool {
        self.run.len() > self.selected_segments.len()
    }

    /// Removes all the selected segments, unless all of them are selected. The
    /// run's information is automatically adjusted properly. The next
    /// not-to-be-removed segment after the active segment becomes the new
    /// active segment. If there's none, then the next not-to-be-removed segment
    /// before the active segment, becomes the new active segment.
    pub fn remove_segments(&mut self) {
        if !self.can_remove_segments() {
            return;
        }

        let mut removed = 0;
        for i in 0..self.run.len() {
            if self.selected_segments.contains(&i) {
                let segment_index = i - removed;
                self.fix_after_deletion(segment_index);
                self.run.segments_mut().remove(segment_index);
                removed += 1;
            }
        }

        let selected_segment = self.active_segment_index();
        let above_count = self
            .selected_segments
            .iter()
            .filter(|&&i| i < selected_segment)
            .count();
        let mut new_index = selected_segment - above_count;
        if new_index >= self.run.len() {
            new_index = self.run.len() - 1;
        }
        self.selected_segments.clear();
        self.selected_segments.push(new_index);

        self.times_modified();
        self.fix();
    }

    fn switch_segments(&mut self, index: usize) {
        let max_index = self.run.max_attempt_history_index().unwrap_or(0);
        let min_index = self.run.min_segment_history_index().unwrap();

        // Use split_at to prove that the 3 segments are distinct
        let (a, b) = self.run.segments_mut().split_at_mut(index);
        let previous = a.last();
        let (a, b) = b.split_at_mut(1);
        let first = &mut a[0];
        let second = &mut b[0];

        for run_index in min_index..=max_index {
            // Remove both segment history elements if one of them has a None
            // time and the other has has a non None time
            let first_history = first.segment_history().get(run_index);
            let second_history = second.segment_history().get(run_index);
            if let (Some(first_history), Some(second_history)) = (first_history, second_history) {
                if first_history.real_time.is_some() != second_history.real_time.is_some()
                    || first_history.game_time.is_some() != second_history.game_time.is_some()
                {
                    first.segment_history_mut().remove(run_index);
                    second.segment_history_mut().remove(run_index);
                }
            }
        }

        for (comparison, first_time) in first.comparisons_mut().iter_mut() {
            // Fix the comparison times based on the new positions of the two
            // segments
            let previous_time = previous
                .map(|p| p.comparison(comparison))
                .unwrap_or_else(Time::zero);

            let second_time = second.comparison_mut(comparison);
            let first_segment_time = *first_time - previous_time;
            let second_segment_time = *second_time - *first_time;
            *second_time = previous_time + second_segment_time;
            *first_time = *second_time + first_segment_time;
        }

        swap(first, second);
    }

    /// Checks if the currently selected segments can be moved up. If any one of
    /// the selected segments is the first segment, then they can't be moved.
    pub fn can_move_segments_up(&self) -> bool {
        !self.selected_segments.iter().any(|&s| s == 0)
    }

    /// Moves all the selected segments up, unless the first segment is
    /// selected. The run's information is automatically adjusted properly. The
    /// active segment stays the active segment.
    pub fn move_segments_up(&mut self) {
        if !self.can_move_segments_up() {
            return;
        }

        for i in 0..self.run.len() - 1 {
            if self.selected_segments.contains(&(i + 1)) {
                self.switch_segments(i);
            }
        }

        for segment in &mut self.selected_segments {
            *segment = segment.saturating_sub(1);
        }

        self.times_modified();
        self.fix();
    }

    /// Checks if the currently selected segments can be moved down. If any one
    /// of the selected segments is the last segment, then they can't be moved.
    pub fn can_move_segments_down(&self) -> bool {
        let last_index = self.run.len() - 1;
        !self.selected_segments.iter().any(|&s| s == last_index)
    }

    /// Moves all the selected segments down, unless the last segment is
    /// selected. The run's information is automatically adjusted properly. The
    /// active segment stays the active segment.
    pub fn move_segments_down(&mut self) {
        if !self.can_move_segments_down() {
            return;
        }

        for i in (0..self.run.len() - 1).rev() {
            if self.selected_segments.contains(&i) {
                self.switch_segments(i);
            }
        }

        for segment in &mut self.selected_segments {
            if *segment < self.run.len() - 1 {
                *segment += 1;
            }
        }

        self.times_modified();
        self.fix();
    }

    /// Adds a new custom comparison. It can't be added if it starts with
    /// `[Race]` or already exists.
    pub fn add_comparison<S: PopulateString>(&mut self, comparison: S) -> ComparisonResult<()> {
        self.run.add_custom_comparison(comparison)?;
        self.fix();
        Ok(())
    }

    /// Imports the Personal Best from the provided run as a comparison. The
    /// comparison can't be added if its name starts with `[Race]` or it already
    /// exists.
    pub fn import_comparison(&mut self, run: &Run, comparison: &str) -> ComparisonResult<()> {
        self.run.add_custom_comparison(comparison)?;

        let mut remaining_segments = self.run.segments_mut().as_mut_slice();

        for segment in run.segments().iter().take(run.len().saturating_sub(1)) {
            if let Some((segment_index, my_segment)) = remaining_segments
                .iter_mut()
                .enumerate()
                .find(|(_, s)| unicase::eq(segment.name(), s.name()))
            {
                *my_segment.comparison_mut(comparison) = segment.personal_best_split_time();
                remaining_segments = &mut remaining_segments[segment_index + 1..];
            }
        }

        if let (Some(my_segment), Some(segment)) =
            (self.run.segments_mut().last_mut(), run.segments().last())
        {
            *my_segment.comparison_mut(comparison) = segment.personal_best_split_time();
        }

        self.fix();
        Ok(())
    }

    /// Removes the chosen custom comparison. You can't remove a Comparison
    /// Generator's Comparison or the Personal Best.
    pub fn remove_comparison(&mut self, comparison: &str) {
        if comparison == comparison::personal_best::NAME {
            return;
        }

        self.run
            .custom_comparisons_mut()
            .retain(|c| c != comparison);

        if self.run.comparisons().any(|c| c == comparison) {
            return;
        }

        for segment in self.run.segments_mut() {
            segment.comparisons_mut().remove(comparison);
        }

        self.fix();
    }

    /// Renames a comparison. The comparison can't be renamed if the new name of
    /// the comparison starts with `[Race]` or it already exists.
    pub fn rename_comparison(&mut self, old: &str, new: &str) -> Result<(), RenameError> {
        if old == new {
            return Ok(());
        }

        self.run
            .validate_comparison_name(new)
            .context(InvalidName)?;

        {
            let comparison_name = self
                .run
                .custom_comparisons_mut()
                .iter_mut()
                .find(|c| *c == old)
                .context(OldNameNotFound)?;

            comparison_name.clear();
            comparison_name.push_str(new);
        }

        for segment in self.run.segments_mut() {
            if let Some(time) = segment.comparisons_mut().remove(old) {
                *segment.comparison_mut(new) = time;
            }
        }

        self.fix();

        Ok(())
    }

    /// Reorders the custom comparisons by moving the comparison with the
    /// `src_index` specified to the `dst_index` specified. Returns `Err(())` if
    /// one of the indices is invalid. The indices are based on the
    /// `comparison_names` field of the Run Editor's `State`.
    pub fn move_comparison(&mut self, src_index: usize, dst_index: usize) -> Result<(), ()> {
        let comparisons = self.run.custom_comparisons_mut();
        let (src_index, dst_index) = (src_index + 1, dst_index + 1);
        if src_index >= comparisons.len() || dst_index >= comparisons.len() {
            return Err(());
        }
        if src_index == dst_index {
            return Ok(());
        }

        if src_index > dst_index {
            comparisons[dst_index..=src_index].rotate_left(src_index - dst_index);
        } else {
            comparisons[src_index..=dst_index].rotate_left(1);
        }

        self.raise_run_edited();
        Ok(())
    }

    /// Generates a custom goal comparison based on the goal time provided. The
    /// comparison's times are automatically balanced based on the runner's
    /// history such that it roughly represents what split times for the goal
    /// time would roughly look like. Since it is populated by the runner's
    /// history, only goal times within the sum of the best segments and the sum
    /// of the worst segments are supported. Everything else is automatically
    /// capped by that range. The comparison is only populated for the selected
    /// timing method. The other timing method's comparison times are not
    /// modified by this, so you can call this again with the other timing
    /// method to generate the comparison times for both timing methods.
    pub fn generate_goal_comparison(&mut self, time: TimeSpan) {
        if !self
            .run
            .custom_comparisons()
            .iter()
            .any(|c| c == comparison::goal::NAME)
        {
            self.run
                .custom_comparisons_mut()
                .push(String::from(comparison::goal::NAME));
        }

        comparison::goal::generate_for_timing_method(
            self.run.segments_mut(),
            self.selected_method,
            time,
            comparison::goal::NAME,
        );

        self.raise_run_edited();
    }

    /// Parses a goal time and generates a custom goal comparison based on the
    /// parsed value. The comparison's times are automatically balanced based on
    /// the runner's history such that it roughly represents what split times
    /// for the goal time would roughly look like. Since it is populated by the
    /// runner's history, only goal times within the sum of the best segments
    /// and the sum of the worst segments are supported. Everything else is
    /// automatically capped by that range. The comparison is only populated for
    /// the selected timing method. The other timing method's comparison times
    /// are not modified by this, so you can call this again with the other
    /// timing method to generate the comparison times for both timing methods.
    pub fn parse_and_generate_goal_comparison(&mut self, time: &str) -> Result<(), ParseError> {
        self.generate_goal_comparison(
            parse_positive(time)?.ok_or(ParseError::EmptyTimeNotAllowed)?,
        );
        Ok(())
    }

    /// Clears out the Attempt History and the Segment Histories of all the
    /// segments.
    pub fn clear_history(&mut self) {
        self.run.clear_history();
        self.fix();
    }

    /// Clears out the Attempt History, the Segment Histories, all the times,
    /// sets the Attempt Count to 0 and clears the speedrun.com run id
    /// association. All Custom Comparisons other than `Personal Best` are
    /// deleted as well.
    pub fn clear_times(&mut self) {
        self.run.clear_times();
        self.fix();
    }

    /// Creates a Sum of Best Cleaner which allows you to interactively remove
    /// potential issues in the segment history that lead to an inaccurate Sum
    /// of Best. If you skip a split, whenever you will do the next split, the
    /// combined segment time might be faster than the sum of the individual
    /// best segments. The Sum of Best Cleaner will point out all of these and
    /// allows you to delete them individually if any of them seem wrong.
    pub fn clean_sum_of_best(&mut self) -> SumOfBestCleaner<'_> {
        SumOfBestCleaner::new(&mut self.run)
    }
}

fn parse_positive(time: &str) -> Result<Option<TimeSpan>, ParseError> {
    let time = TimeSpan::parse_opt(time).context(ParseTime)?;
    if time.map_or(false, |t| t < TimeSpan::zero()) {
        Err(ParseError::NegativeTimeNotAllowed)
    } else {
        Ok(time)
    }
}
