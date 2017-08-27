use {Image, TimeSpan};
use super::{Editor, ParseError};

pub struct SegmentRow<'editor> {
    index: usize,
    editor: &'editor mut Editor,
}

impl<'a> SegmentRow<'a> {
    pub fn new(index: usize, editor: &'a mut Editor) -> Self {
        SegmentRow {
            index: index,
            editor: editor,
        }
    }

    pub fn icon(&self) -> &Image {
        &self.editor.run.segments[self.index].icon
    }

    pub fn set_icon<D: Into<Image>>(&mut self, image: D) {
        self.editor.run.segments[self.index].icon = image.into();
        self.editor.raise_run_edited();
    }

    pub fn remove_icon(&mut self) {
        self.editor.run.segments[self.index].icon = Image::default();
        self.editor.raise_run_edited();
    }

    pub fn name(&self) -> &str {
        &self.editor.run.segments[self.index].name
    }

    pub fn set_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.editor.run.segments[self.index].name.clear();
        self.editor.run.segments[self.index].name.push_str(name.as_ref());
        self.editor.raise_run_edited();
    }

    pub fn split_time(&self) -> Option<TimeSpan> {
        let method = self.editor.selected_method;
        self.editor
            .run
            .segments[self.index]
            .personal_best_split_time()[method]
    }

    pub fn set_split_time(&mut self, time: Option<TimeSpan>) {
        let method = self.editor.selected_method;
        self.editor
            .run
            .segments[self.index]
            .personal_best_split_time_mut()[method] = time;
        self.editor.times_modified();
        self.editor.fix();
    }

    pub fn parse_and_set_split_time<S>(&mut self, time: S) -> Result<(), ParseError>
    where
        S: AsRef<str>,
    {
        self.set_split_time(parse_positive(time)?);
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

    pub fn parse_and_set_segment_time<S>(&mut self, time: S) -> Result<(), ParseError>
    where
        S: AsRef<str>,
    {
        self.set_segment_time(parse_positive(time)?);
        Ok(())
    }

    pub fn best_segment_time(&self) -> Option<TimeSpan> {
        let method = self.editor.selected_method;
        self.editor.run.segments[self.index].best_segment_time[method]
    }

    pub fn set_best_segment_time(&mut self, time: Option<TimeSpan>) {
        let method = self.editor.selected_method;
        self.editor
            .run
            .segments[self.index]
            .best_segment_time[method] = time;
        self.editor.times_modified();
        self.editor.fix();
    }

    pub fn parse_and_set_best_segment_time<S>(&mut self, time: S) -> Result<(), ParseError>
    where
        S: AsRef<str>,
    {
        self.set_best_segment_time(parse_positive(time)?);
        Ok(())
    }

    pub fn comparison_time(&self, comparison: &str) -> Option<TimeSpan> {
        let method = self.editor.selected_method;
        self.editor.run.segments[self.index].comparison(comparison)[method]
    }

    pub fn set_comparison_time(&mut self, comparison: &str, time: Option<TimeSpan>) {
        let method = self.editor.selected_method;
        self.editor
            .run
            .segments[self.index]
            .comparison_mut(comparison)[method] = time;
        self.editor.times_modified();
        self.editor.fix();
    }

    pub fn parse_and_set_comparison_time<S>(
        &mut self,
        comparison: &str,
        time: S,
    ) -> Result<(), ParseError>
    where
        S: AsRef<str>,
    {
        self.set_comparison_time(comparison, parse_positive(time)?);
        Ok(())
    }
}

fn parse_positive<S>(time: S) -> Result<Option<TimeSpan>, ParseError>
where
    S: AsRef<str>,
{
    let time = TimeSpan::parse_opt(time)?;
    if time.map_or(false, |t| t < TimeSpan::zero()) {
        Err(ParseError::NegativeTimeNotAllowed)
    } else {
        Ok(time)
    }
}
