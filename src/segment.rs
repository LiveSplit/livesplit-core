use std::collections::HashMap;
use {SegmentHistory, Image, Time, TimingMethod, TimeSpan};
use comparison::personal_best;

#[derive(Clone, Default, Debug)]
pub struct Segment {
    name: String,
    icon: Image,
    best_segment_time: Time,
    split_time: Time,
    segment_history: SegmentHistory,
    comparisons: HashMap<String, Time>,
}

impl Segment {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Segment {
            name: name.into(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn set_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.name.clear();
        self.name.push_str(name.as_ref());
    }

    #[inline]
    pub fn icon(&self) -> &Image {
        &self.icon
    }

    #[inline]
    pub fn set_icon<D: Into<Image>>(&mut self, image: D) {
        self.icon = image.into();
    }

    #[inline]
    pub fn comparisons_mut(&mut self) -> &mut HashMap<String, Time> {
        &mut self.comparisons
    }

    #[inline]
    pub fn comparison_mut(&mut self, comparison: &str) -> &mut Time {
        self.comparisons
            .entry(comparison.into())
            .or_insert_with(Time::default)
    }

    #[inline]
    pub fn comparison(&self, comparison: &str) -> Time {
        self.comparisons
            .get(comparison)
            .cloned()
            .unwrap_or_default()
    }

    #[inline]
    pub fn comparison_timing_method(
        &self,
        comparison: &str,
        method: TimingMethod,
    ) -> Option<TimeSpan> {
        self.comparisons.get(comparison).and_then(|t| t[method])
    }

    #[inline]
    pub fn personal_best_split_time(&self) -> Time {
        self.comparisons
            .get(personal_best::NAME)
            .cloned()
            .unwrap_or_else(Time::default)
    }

    #[inline]
    pub fn personal_best_split_time_mut(&mut self) -> &mut Time {
        self.comparisons
            .entry(personal_best::NAME.to_string())
            .or_insert_with(Time::default)
    }

    #[inline]
    pub fn set_personal_best_split_time(&mut self, time: Time) {
        self.comparisons.insert(personal_best::NAME.into(), time);
    }

    #[inline]
    pub fn best_segment_time(&self) -> Time {
        self.best_segment_time
    }

    #[inline]
    pub fn best_segment_time_mut(&mut self) -> &mut Time {
        &mut self.best_segment_time
    }

    #[inline]
    pub fn set_best_segment_time(&mut self, time: Time) {
        self.best_segment_time = time;
    }

    #[inline]
    pub fn split_time(&self) -> Time {
        self.split_time
    }

    #[inline]
    pub fn split_time_mut(&mut self) -> &mut Time {
        &mut self.split_time
    }

    #[inline]
    pub fn set_split_time(&mut self, time: Time) {
        self.split_time = time;
    }

    #[inline]
    pub fn clear_split_time(&mut self) {
        self.set_split_time(Default::default());
    }

    #[inline]
    pub fn segment_history(&self) -> &SegmentHistory {
        &self.segment_history
    }

    #[inline]
    pub fn segment_history_mut(&mut self) -> &mut SegmentHistory {
        &mut self.segment_history
    }
}
