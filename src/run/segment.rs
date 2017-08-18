use std::collections::HashMap;
use {Image, SegmentHistory, Time, TimeSpan, TimingMethod};
use comparison::personal_best;

#[derive(Clone, Default, Debug)]
pub struct Segment {
    pub name: String,
    pub icon: Image,
    pub best_segment_time: Time,
    pub split_time: Time,
    pub segment_history: SegmentHistory,
    pub comparisons: HashMap<String, Time>,
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
    pub fn clear_split_time(&mut self) {
        self.split_time = Default::default();
    }
}
