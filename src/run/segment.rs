use crate::comparison::personal_best;
use crate::platform::prelude::*;
use crate::{settings::Image, SegmentHistory, Time, TimeSpan, TimingMethod};
use hashbrown::HashMap;

/// A Segment describes a point in a speedrun that is suitable for storing a
/// split time. This stores the name of that segment, an icon, the split times
/// of different comparisons, and a history of segment times.
///
/// # Examples
///
/// ```
/// use livesplit_core::{Segment, Time, TimeSpan};
///
/// let mut segment = Segment::new("Metro Kingdom");
///
/// let time = Time::new().with_real_time(Some(TimeSpan::from_seconds(234.0)));
/// segment.set_personal_best_split_time(time);
/// ```
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Segment {
    name: String,
    icon: Image,
    best_segment_time: Time,
    split_time: Time,
    segment_history: SegmentHistory,
    comparisons: HashMap<String, Time>,
}

impl Segment {
    /// Creates a new Segment with the name given.
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Segment {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Accesses the name of the segment.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the segment.
    #[inline]
    pub fn set_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.name.clear();
        self.name.push_str(name.as_ref());
    }

    /// Accesses the icon of the segment.
    #[inline]
    pub fn icon(&self) -> &Image {
        &self.icon
    }

    /// Sets the icon of the segment.
    #[inline]
    pub fn set_icon<D: Into<Image>>(&mut self, image: D) {
        self.icon = image.into();
    }

    /// Grants mutable access to the comparison times stored in the Segment.
    /// This includes both the custom comparisons and the generated ones.
    #[inline]
    pub fn comparisons_mut(&mut self) -> &mut HashMap<String, Time> {
        &mut self.comparisons
    }

    /// Grants mutable access to the specified comparison's time. If there's
    /// none for this comparison, a new one is inserted with an empty time.
    #[inline]
    pub fn comparison_mut(&mut self, comparison: &str) -> &mut Time {
        self.comparisons
            .entry(comparison.into())
            .or_insert_with(Time::default)
    }

    /// Accesses the specified comparison's time. If there's none for this
    /// comparison, an empty time is being returned (but not stored in the
    /// segment).
    #[inline]
    pub fn comparison(&self, comparison: &str) -> Time {
        self.comparisons
            .get(comparison)
            .cloned()
            .unwrap_or_default()
    }

    /// Accesses the given timing method of the specified comparison. If either
    /// the TimeSpan is empty or the comparison has no stored time, `None` is
    /// returned.
    #[inline]
    pub fn comparison_timing_method(
        &self,
        comparison: &str,
        method: TimingMethod,
    ) -> Option<TimeSpan> {
        self.comparisons.get(comparison).and_then(|t| t[method])
    }

    /// Accesses the split time of the Personal Best for this segment. If it
    /// doesn't exist, an empty time is returned.
    #[inline]
    pub fn personal_best_split_time(&self) -> Time {
        self.comparisons
            .get(personal_best::NAME)
            .cloned()
            .unwrap_or_else(Time::default)
    }

    /// Grants mutable access to the split time of the Personal Best for this
    /// segment. If it doesn't exist an empty time is inserted.
    #[inline]
    pub fn personal_best_split_time_mut(&mut self) -> &mut Time {
        self.comparisons
            .entry(personal_best::NAME.to_string())
            .or_insert_with(Time::default)
    }

    /// Sets the split time of the Personal Best to the time provided.
    #[inline]
    pub fn set_personal_best_split_time(&mut self, time: Time) {
        self.comparisons.insert(personal_best::NAME.into(), time);
    }

    /// Accesses the Best Segment Time.
    #[inline]
    pub fn best_segment_time(&self) -> Time {
        self.best_segment_time
    }

    /// Grants mutable access to the Best Segment Time.
    #[inline]
    pub fn best_segment_time_mut(&mut self) -> &mut Time {
        &mut self.best_segment_time
    }

    /// Sets the Best Segment Time.
    #[inline]
    pub fn set_best_segment_time(&mut self, time: Time) {
        self.best_segment_time = time;
    }

    /// Accesses the split time of the current attempt.
    #[inline]
    pub fn split_time(&self) -> Time {
        self.split_time
    }

    /// Grants mutable access to the split time of the current attempt.
    #[inline]
    pub fn split_time_mut(&mut self) -> &mut Time {
        &mut self.split_time
    }

    /// Sets the split time of the current attempt.
    #[inline]
    pub fn set_split_time(&mut self, time: Time) {
        self.split_time = time;
    }

    /// Clears the split time of the current attempt.
    #[inline]
    pub fn clear_split_time(&mut self) {
        self.set_split_time(Default::default());
    }

    /// Accesses the Segment History of this segment.
    #[inline]
    pub fn segment_history(&self) -> &SegmentHistory {
        &self.segment_history
    }

    /// Grants mutable access to the Segment History of this segment.
    #[inline]
    pub fn segment_history_mut(&mut self) -> &mut SegmentHistory {
        &mut self.segment_history
    }
}
