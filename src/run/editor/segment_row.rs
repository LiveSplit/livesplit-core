use super::{parse_positive, Editor, ParseError};
use crate::{settings::Image, util::PopulateString, TimeSpan};

/// A Segment Row describes the segment in the Run Editor actively selected for
/// editing.
pub struct SegmentRow<'editor> {
    index: usize,
    editor: &'editor mut Editor,
}

impl<'a> SegmentRow<'a> {
    pub(super) fn new(index: usize, editor: &'a mut Editor) -> Self {
        SegmentRow { index, editor }
    }

    /// Accesses the icon of the segment.
    pub fn icon(&self) -> &Image {
        self.editor.run.segment(self.index).icon()
    }

    /// Sets the icon of the segment.
    pub fn set_icon<D: Into<Image>>(&mut self, image: D) {
        self.editor.run.segment_mut(self.index).set_icon(image);
        self.editor.raise_run_edited();
    }

    /// Removes the icon of the segment.
    pub fn remove_icon(&mut self) {
        self.editor.run.segment_mut(self.index).set_icon([]);
        self.editor.raise_run_edited();
    }

    /// Accesses the name of the segment.
    pub fn name(&self) -> &str {
        self.editor.run.segment(self.index).name()
    }

    /// Sets the name of the segment.
    pub fn set_name<S>(&mut self, name: S)
    where
        S: PopulateString,
    {
        self.editor.run.segment_mut(self.index).set_name(name);
        self.editor.raise_run_edited();
    }

    /// Accesses the split time of the segment for the active timing method.
    pub fn split_time(&self) -> Option<TimeSpan> {
        let method = self.editor.selected_method;
        self.editor
            .run
            .segment(self.index)
            .personal_best_split_time()[method]
    }

    /// Sets the split time of the segment for the active timing method.
    pub fn set_split_time(&mut self, time: Option<TimeSpan>) {
        let method = self.editor.selected_method;
        self.editor
            .run
            .segment_mut(self.index)
            .personal_best_split_time_mut()[method] = time;
        self.editor.times_modified();
        self.editor.fix();
    }

    /// Parses a split time from a string and sets it for the active timing
    /// method.
    pub fn parse_and_set_split_time(&mut self, time: &str) -> Result<(), ParseError> {
        self.set_split_time(parse_positive(time)?);
        Ok(())
    }

    /// Accesses the segment time of the segment for the active timing method.
    pub fn segment_time(&self) -> Option<TimeSpan> {
        self.editor.segment_times[self.index]
    }

    /// Sets the segment time of the segment for the active timing method.
    pub fn set_segment_time(&mut self, time: Option<TimeSpan>) {
        self.editor.segment_times[self.index] = time;
        self.editor.fix_splits_from_segments();
        self.editor.times_modified();
        self.editor.fix();
    }

    /// Parses a segment time from a string and sets it for the active timing
    /// method.
    pub fn parse_and_set_segment_time(&mut self, time: &str) -> Result<(), ParseError> {
        self.set_segment_time(parse_positive(time)?);
        Ok(())
    }

    /// Accesses the best segment time of the segment for the active timing method.
    pub fn best_segment_time(&self) -> Option<TimeSpan> {
        let method = self.editor.selected_method;
        self.editor.run.segment(self.index).best_segment_time()[method]
    }

    /// Sets the best segment time of the segment for the active timing method.
    pub fn set_best_segment_time(&mut self, time: Option<TimeSpan>) {
        let method = self.editor.selected_method;
        self.editor
            .run
            .segment_mut(self.index)
            .best_segment_time_mut()[method] = time;
        self.editor.times_modified();
        self.editor.fix();
    }

    /// Parses a best segment time from a string and sets it for the active
    /// timing method.
    pub fn parse_and_set_best_segment_time(&mut self, time: &str) -> Result<(), ParseError> {
        self.set_best_segment_time(parse_positive(time)?);
        Ok(())
    }

    /// Accesses the provided comparison's time of the segment for the active
    /// timing method.
    pub fn comparison_time(&self, comparison: &str) -> Option<TimeSpan> {
        let method = self.editor.selected_method;
        self.editor.run.segment(self.index).comparison(comparison)[method]
    }

    /// Sets the provided comparison's time of the segment for the active timing method.
    pub fn set_comparison_time(&mut self, comparison: &str, time: Option<TimeSpan>) {
        let method = self.editor.selected_method;
        self.editor
            .run
            .segment_mut(self.index)
            .comparison_mut(comparison)[method] = time;
        self.editor.times_modified();
        self.editor.fix();
    }

    /// Parses a comparison time for the provided comparison and sets it for the
    /// active timing method.
    pub fn parse_and_set_comparison_time(
        &mut self,
        comparison: &str,
        time: &str,
    ) -> Result<(), ParseError> {
        self.set_comparison_time(comparison, parse_positive(time)?);
        Ok(())
    }
}
