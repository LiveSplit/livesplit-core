use hashbrown::HashMap;

use super::Comparisons;
use crate::{
    SegmentHistory, Time, TimeSpan, TimingMethod, comparison::personal_best, platform::prelude::*,
    settings::Image, util::PopulateString,
};

/// A `Segment` describes a point in a speedrun that is suitable for storing a
/// split time. This stores the name of that `Segment`, an icon, the split times
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
    comparisons: Comparisons,
    variables: HashMap<String, String>,
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
    #[allow(clippy::missing_const_for_fn)] // FIXME: Can't reason about Deref
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the segment.
    #[inline]
    pub fn set_name<S>(&mut self, name: S)
    where
        S: PopulateString,
    {
        name.populate(&mut self.name);
    }

    /// Accesses the icon of the segment.
    #[inline]
    pub const fn icon(&self) -> &Image {
        &self.icon
    }

    /// Sets the icon of the segment.
    #[inline]
    pub fn set_icon(&mut self, image: Image) {
        self.icon = image;
    }

    /// Grants mutable access to the comparison times stored in the Segment.
    /// This includes both the custom comparisons and the generated ones.
    #[inline]
    pub const fn comparisons_mut(&mut self) -> &mut Comparisons {
        &mut self.comparisons
    }

    /// Grants mutable access to the specified comparison's time. If there's
    /// none for this comparison, a new one is inserted with an empty time.
    #[inline]
    pub fn comparison_mut(&mut self, comparison: &str) -> &mut Time {
        self.comparisons.get_or_insert_default(comparison)
    }

    /// Accesses the specified comparison's time. If there's none for this
    /// comparison, an empty time is being returned (but not stored in the
    /// segment).
    #[inline]
    pub fn comparison(&self, comparison: &str) -> Time {
        self.comparisons.get(comparison).unwrap_or_default()
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
            .unwrap_or_default()
    }

    /// Grants mutable access to the split time of the Personal Best for this
    /// segment. If it doesn't exist an empty time is inserted.
    #[inline]
    pub fn personal_best_split_time_mut(&mut self) -> &mut Time {
        self.comparisons.get_or_insert_default(personal_best::NAME)
    }

    /// Sets the split time of the Personal Best to the time provided.
    #[inline]
    pub fn set_personal_best_split_time(&mut self, time: Time) {
        self.comparisons.set(personal_best::NAME, time);
    }

    /// Accesses the Best Segment Time.
    #[inline]
    pub const fn best_segment_time(&self) -> Time {
        self.best_segment_time
    }

    /// Grants mutable access to the Best Segment Time.
    #[inline]
    pub const fn best_segment_time_mut(&mut self) -> &mut Time {
        &mut self.best_segment_time
    }

    /// Sets the Best Segment Time.
    #[inline]
    pub const fn set_best_segment_time(&mut self, time: Time) {
        self.best_segment_time = time;
    }

    /// Accesses the split time of the current attempt.
    #[inline]
    pub const fn split_time(&self) -> Time {
        self.split_time
    }

    /// Grants mutable access to the split time of the current attempt.
    #[inline]
    pub const fn split_time_mut(&mut self) -> &mut Time {
        &mut self.split_time
    }

    /// Sets the split time of the current attempt.
    #[inline]
    pub const fn set_split_time(&mut self, time: Time) {
        self.split_time = time;
    }

    /// Clears the split time of the current attempt.
    #[inline]
    pub fn clear_split_time(&mut self) {
        self.set_split_time(Default::default());
    }

    /// Accesses the Segment History of this segment.
    #[inline]
    pub const fn segment_history(&self) -> &SegmentHistory {
        &self.segment_history
    }

    /// Grants mutable access to the Segment History of this segment.
    #[inline]
    pub const fn segment_history_mut(&mut self) -> &mut SegmentHistory {
        &mut self.segment_history
    }

    /// Accesses the segment's variables for the current attempt.
    pub const fn variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    /// Grants mutable access to the segment's variables for the current
    /// attempt.
    pub const fn variables_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.variables
    }

    /// Clears the variables of the current attempt.
    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }

    /// Clears all the information the segment stores when it has been splitted,
    /// such as the split's time and variables.
    pub fn clear_split_info(&mut self) {
        self.clear_variables();
        self.clear_split_time();
    }
}
