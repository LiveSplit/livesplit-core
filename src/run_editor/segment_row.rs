use {Image, TimeSpan};
use time_span::ParseError as ParseTimeSpanError;
use super::RunEditor;

pub struct SegmentRow<'editor> {
    index: usize,
    editor: &'editor mut RunEditor,
}

impl<'a> SegmentRow<'a> {
    pub fn new(index: usize, editor: &'a mut RunEditor) -> Self {
        SegmentRow {
            index: index,
            editor: editor,
        }
    }

    pub fn icon(&self) -> &Image {
        self.editor.run.segment(self.index).icon()
    }

    pub fn set_icon<D: Into<Image>>(&mut self, image: D) {
        self.editor.run.segment_mut(self.index).set_icon(image);
        self.editor.raise_run_edited();
    }

    pub fn name(&self) -> &str {
        self.editor.run.segment(self.index).name()
    }

    pub fn set_name<S>(&mut self, name: S)
        where S: AsRef<str>
    {
        self.editor.run.segment_mut(self.index).set_name(name);
        self.editor.raise_run_edited();
    }

    pub fn split_time(&self) -> Option<TimeSpan> {
        let method = self.editor.selected_method;
        self.editor.run.segment(self.index).personal_best_split_time()[method]
    }

    pub fn set_split_time(&mut self, time: Option<TimeSpan>) {
        let method = self.editor.selected_method;
        self.editor.run.segment_mut(self.index).personal_best_split_time_mut()[method] = time;
        self.editor.times_modified();
        self.editor.fix();
    }

    pub fn parse_and_set_split_time<S>(&mut self, time: S) -> Result<(), ParseTimeSpanError>
        where S: AsRef<str>
    {
        self.set_split_time(TimeSpan::parse_opt(time)?);
        Ok(())
    }

    pub fn segment_time(&self) -> Option<TimeSpan> {
        self.editor.segment_times[self.index]
    }

    pub fn set_segment_time(&mut self, time: Option<TimeSpan>) {
        self.editor.segment_times[self.index] = time;
        self.editor.fix_splits_from_segments();
        self.editor.times_modified();
        self.editor.fix();
    }

    pub fn parse_and_set_segment_time<S>(&mut self, time: S) -> Result<(), ParseTimeSpanError>
        where S: AsRef<str>
    {
        self.set_segment_time(TimeSpan::parse_opt(time)?);
        Ok(())
    }

    pub fn best_segment_time(&self) -> Option<TimeSpan> {
        let method = self.editor.selected_method;
        self.editor.run.segment(self.index).best_segment_time()[method]
    }

    pub fn set_best_segment_time(&mut self, time: Option<TimeSpan>) {
        let method = self.editor.selected_method;
        self.editor.run.segment_mut(self.index).best_segment_time_mut()[method] = time;
        self.editor.times_modified();
        self.editor.fix();
    }

    pub fn parse_and_set_best_segment_time<S>(&mut self, time: S) -> Result<(), ParseTimeSpanError>
        where S: AsRef<str>
    {
        self.set_best_segment_time(TimeSpan::parse_opt(time)?);
        Ok(())
    }

    pub fn comparison_time(&self, comparison: &str) -> Option<TimeSpan> {
        let method = self.editor.selected_method;
        self.editor.run.segment(self.index).comparison(comparison)[method]
    }

    pub fn set_comparison_time(&mut self, comparison: &str, time: Option<TimeSpan>) {
        let method = self.editor.selected_method;
        self.editor.run.segment_mut(self.index).comparison_mut(comparison)[method] = time;
        self.editor.times_modified();
        self.editor.fix();
    }

    pub fn parse_and_set_comparison_time<S>(&mut self,
                                            comparison: &str,
                                            time: S)
                                            -> Result<(), ParseTimeSpanError>
        where S: AsRef<str>
    {
        self.set_comparison_time(comparison, TimeSpan::parse_opt(time)?);
        Ok(())
    }
}
