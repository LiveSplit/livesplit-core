use std::num::ParseIntError;
use std::mem::swap;
use {unicase, Image, Run, Segment, Time, TimeSpan, TimingMethod};
use time::ParseError as ParseTimeSpanError;

mod segment_row;
mod state;
#[cfg(test)]
mod tests;

pub use self::segment_row::SegmentRow;
pub use self::state::{Buttons as ButtonsState, Segment as SegmentState, State};

quick_error! {
    #[derive(Debug)]
    pub enum ParseError {
        TimeSpan(err: ParseTimeSpanError) {
            from()
        }
        NegativeTimeNotAllowed
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum OpenError {
        EmptyRun
    }
}

pub struct Editor {
    run: Run,
    selected_method: TimingMethod,
    selected_segments: Vec<usize>,
    previous_personal_best_time: Time,
    game_icon_id: usize,
    segment_icon_ids: Vec<usize>,
    segment_times: Vec<Option<TimeSpan>>,
}

impl Editor {
    pub fn new(run: Run) -> Result<Self, OpenError> {
        let len = run.len();
        if len == 0 {
            return Err(OpenError::EmptyRun);
        }

        let personal_best_time = run.segments().last().unwrap().personal_best_split_time();

        let mut editor = Self {
            run: run,
            selected_method: TimingMethod::RealTime,
            selected_segments: vec![0],
            previous_personal_best_time: personal_best_time,
            game_icon_id: 0,
            segment_icon_ids: Vec::with_capacity(len),
            segment_times: Vec::with_capacity(len),
        };

        editor.update_segment_list();

        Ok(editor)
    }

    pub fn close(self) -> Run {
        self.run
    }

    pub fn selected_timing_method(&self) -> TimingMethod {
        self.selected_method
    }

    pub fn select_timing_method(&mut self, method: TimingMethod) {
        self.selected_method = method;
        self.update_segment_list();
    }

    fn selected_segment_index(&self) -> usize {
        *self.selected_segments.last().unwrap()
    }

    pub fn selected_segment(&mut self) -> SegmentRow {
        SegmentRow::new(self.selected_segment_index(), self)
    }

    pub fn unselect(&mut self, index: usize) {
        self.selected_segments.retain(|&i| i != index);
        if self.selected_segments.is_empty() {
            self.selected_segments.push(index);
        }
    }

    pub fn select_additionally(&mut self, index: usize) {
        self.selected_segments.retain(|&i| i != index);
        self.selected_segments.push(index);
    }

    pub fn select_only(&mut self, index: usize) {
        self.selected_segments.clear();
        self.selected_segments.push(index);
    }

    fn raise_run_edited(&mut self) {
        self.run.mark_as_changed();
    }

    pub fn game_name(&self) -> &str {
        self.run.game_name()
    }

    pub fn set_game_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.run.set_game_name(name);
        self.raise_run_edited();
    }

    pub fn category_name(&self) -> &str {
        self.run.category_name()
    }

    pub fn set_category_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.run.set_category_name(name);
        self.raise_run_edited();
    }

    pub fn offset(&self) -> TimeSpan {
        self.run.offset()
    }

    pub fn set_offset(&mut self, offset: TimeSpan) {
        self.run.set_offset(offset);
        self.raise_run_edited();
    }

    pub fn parse_and_set_offset<S>(&mut self, offset: S) -> Result<(), ParseError>
    where
        S: AsRef<str>,
    {
        self.set_offset(offset.as_ref().parse()?);
        Ok(())
    }

    pub fn attempt_count(&self) -> u32 {
        self.run.attempt_count()
    }

    pub fn set_attempt_count(&mut self, attempts: u32) {
        self.run.set_attempt_count(attempts);
        self.raise_run_edited();
    }

    pub fn parse_and_set_attempt_count<S>(&mut self, attempts: S) -> Result<(), ParseIntError>
    where
        S: AsRef<str>,
    {
        self.set_attempt_count(attempts.as_ref().parse()?);
        Ok(())
    }

    pub fn game_icon(&self) -> &Image {
        self.run.game_icon()
    }

    pub fn set_game_icon<D: Into<Image>>(&mut self, image: D) {
        self.run.set_game_icon(image);
        self.raise_run_edited();
    }

    pub fn remove_game_icon(&mut self) {
        self.run.set_game_icon(&[]);
        self.raise_run_edited();
    }

    pub fn custom_comparisons(&self) -> &[String] {
        self.run.custom_comparisons()
    }

    fn times_modified(&mut self) {
        let pb_split_time = self.run
            .segments()
            .last()
            .unwrap()
            .personal_best_split_time();
        if pb_split_time.real_time != self.previous_personal_best_time.real_time ||
            pb_split_time.game_time != self.previous_personal_best_time.game_time
        {
            self.run.metadata_mut().set_run_id("");
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
        let mut previous_time = TimeSpan::zero();
        self.segment_times.clear();
        for segment in self.run.segments() {
            if let Some(time) = segment.personal_best_split_time()[method] {
                self.segment_times.push(Some(time - previous_time));
                previous_time = time;
            } else {
                self.segment_times.push(None);
            }
        }
    }

    fn fix_splits_from_segments(&mut self) {
        let method = self.selected_method;
        let mut previous_time = TimeSpan::zero();
        let mut decrement = TimeSpan::zero();
        for (segment_time, segment) in self.segment_times
            .iter_mut()
            .zip(self.run.segments_mut().iter_mut())
        {
            if let Some(ref mut segment_time) = *segment_time {
                let pb_time = &mut segment.personal_best_split_time_mut()[method];
                if pb_time.is_none() {
                    decrement = *segment_time;
                } else {
                    *segment_time -= decrement;
                    decrement = TimeSpan::zero();
                }
                let new_time = previous_time + *segment_time;
                *pb_time = Some(new_time);
                previous_time = new_time;
            } else {
                if let Some(time) = segment.personal_best_split_time()[method] {
                    previous_time = time;
                }
                segment.personal_best_split_time_mut()[method] = None;
            }
        }
    }

    pub fn insert_segment_above(&mut self) {
        let selected_segment = self.selected_segment_index();

        let mut segment = Segment::new("");
        self.run.import_best_segment(selected_segment);

        let max_index = self.run.max_attempt_history_index().unwrap_or(0);
        let min_index = self.run.min_segment_history_index();
        for x in min_index..max_index + 1 {
            segment.segment_history_mut().insert(x, Default::default());
        }
        self.run.segments_mut().insert(selected_segment, segment);

        self.select_only(selected_segment);

        self.fix();
    }

    pub fn insert_segment_below(&mut self) {
        let selected_segment = self.selected_segment_index();
        let next_segment = selected_segment + 1;

        let mut segment = Segment::new("");
        if next_segment < self.run.len() {
            self.run.import_best_segment(next_segment);
        }

        let max_index = self.run.max_attempt_history_index().unwrap_or(0);
        let min_index = self.run.min_segment_history_index();
        for x in min_index..max_index + 1 {
            segment.segment_history_mut().insert(x, Default::default());
        }
        self.run.segments_mut().insert(next_segment, segment);

        self.select_only(next_segment);

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
        let min_index = self.run.min_segment_history_index();
        for run_index in min_index..max_index + 1 {
            // If a history element isn't there in the segment that's deleted
            // remove it from the next segment's history as well
            if let Some(segment_history_element) =
                self.run.segment(index).segment_history().get(run_index)
            {
                let current_segment = segment_history_element[method];
                if let Some(current_segment) = current_segment {
                    for current_index in current_index..self.run.len() {
                        // Add the removed segment's history times to the next non null times
                        if let Some(&mut Some(ref mut segment)) = self.run
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
        let min_best_segment = TimeSpan::option_add(
            self.run.segment(index).best_segment_time()[method],
            self.run.segment(current_index).best_segment_time()[method],
        );

        if let Some(mut min_best_segment) = min_best_segment {
            // Use any element in the history that has a lower time than this sum
            for time in self.run
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

    pub fn can_remove_segments(&self) -> bool {
        self.run.len() > self.selected_segments.len()
    }

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

        let selected_segment = self.selected_segment_index();
        let above_count = self.selected_segments
            .iter()
            .filter(|&&i| i < selected_segment)
            .count();
        let mut new_index = selected_segment - above_count;
        if new_index >= self.run.len() {
            new_index = self.run.len() - 1;
        }
        self.selected_segments.clear();
        self.selected_segments.push(new_index);

        self.fix();
    }

    fn switch_segments(&mut self, index: usize) {
        let max_index = self.run.max_attempt_history_index().unwrap_or(0);
        let min_index = self.run.min_segment_history_index();

        // Use split_at to prove that the 3 segments are distinct
        let (a, b) = self.run.segments_mut().split_at_mut(index);
        let previous = a.last();
        let (a, b) = b.split_at_mut(1);
        let first = &mut a[0];
        let second = &mut b[0];

        for run_index in min_index..max_index + 1 {
            // Remove both segment history elements if one of them
            // has a null time and the other has has a non null time
            let first_history = first.segment_history().get(run_index);
            let second_history = second.segment_history().get(run_index);
            if let (Some(first_history), Some(second_history)) = (first_history, second_history) {
                if first_history.real_time.is_some() != second_history.real_time.is_some() ||
                    first_history.game_time.is_some() != second_history.game_time.is_some()
                {
                    first.segment_history_mut().remove(run_index);
                    second.segment_history_mut().remove(run_index);
                }
            }
        }

        for (comparison, first_time) in first.comparisons_mut() {
            // Fix the comparison times based on the new positions of the two segments
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

    pub fn can_move_segments_up(&self) -> bool {
        !self.selected_segments.iter().any(|&s| s == 0)
    }

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

        self.fix();
    }

    pub fn can_move_segments_down(&self) -> bool {
        let last_index = self.run.len() - 1;
        !self.selected_segments.iter().any(|&s| s == last_index)
    }

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

        self.fix();
    }

    pub fn add_comparison<S: Into<String>>(&mut self, comparison: S) -> Result<(), ()> {
        let comparison = comparison.into();
        if validate_comparison_name(&self.run, &comparison) {
            self.run.add_custom_comparison(comparison);
            self.fix();
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn import_comparison<S: Into<String>>(
        &mut self,
        run: &Run,
        comparison: S,
    ) -> Result<(), ()> {
        let comparison = comparison.into();
        if validate_comparison_name(&self.run, &comparison) {
            self.run.add_custom_comparison(comparison.as_str());

            for segment in run.segments().iter().take(run.len().saturating_sub(1)) {
                if let Some(my_segment) = self.run
                    .segments_mut()
                    .iter_mut()
                    .find(|s| unicase::eq(segment.name(), s.name()))
                {
                    *my_segment.comparison_mut(&comparison) = segment.personal_best_split_time();
                }
            }

            if let (Some(my_segment), Some(segment)) =
                (self.run.segments_mut().last_mut(), run.segments().last())
            {
                *my_segment.comparison_mut(&comparison) = segment.personal_best_split_time();
            }

            self.fix();
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn remove_comparison(&mut self, comparison: &str) {
        self.run
            .custom_comparisons_mut()
            .retain(|c| c != comparison);

        for segment in self.run.segments_mut() {
            segment.comparisons_mut().remove(comparison);
        }

        self.fix();
    }

    pub fn rename_comparison(&mut self, old: &str, new: &str) -> Result<(), ()> {
        if old == new {
            return Ok(());
        }

        if validate_comparison_name(&self.run, new) {
            let position = self.run
                .custom_comparisons()
                .iter()
                .position(|c| c == old)
                .ok_or(())?;

            self.run.custom_comparisons_mut()[position] = new.to_string();

            for segment in self.run.segments_mut() {
                if let Some(time) = segment.comparisons_mut().remove(old) {
                    *segment.comparison_mut(new) = time;
                }
            }

            self.fix();

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn clear_history(&mut self) {
        self.run.clear_history();
        self.fix();
    }

    pub fn clear_times(&mut self) {
        self.run.clear_times();
        self.fix();
    }
}

fn validate_comparison_name(run: &Run, comparison: &str) -> bool {
    !comparison.starts_with("[Race]") && !run.comparisons().any(|c| c == comparison)
}
