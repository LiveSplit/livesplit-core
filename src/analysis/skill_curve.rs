use crate::platform::prelude::*;
use crate::{Segment, TimeSpan, TimingMethod};
use core::cmp::Ordering;
use ordered_float::OrderedFloat;

const WEIGHT: f64 = 0.75;
const TRIES: usize = 50;

/// The skill curve analyzes the segment history across all segments. For each
/// segment all the segment times are sorted by length and weighted by their
/// recency. Plotting this on a graph with the y-axis representing the segment
/// time and the x-axis representing the percentile, with the shortest time at 0
/// and the longest time at 1, yields the so called "skill curve". If you sum
/// all the different curves together for all the segments, you get the overall
/// curve for the whole run.
///
/// # Properties of the Skill Curve
///
/// If you sample the curve at 0, you get the simple sum of best segments and if
/// you sample the curve at 1, you get the simple sum of worst segments. At 0.5
/// you get the median segments. If you sample the individual segments where you
/// find the Personal Best on the overall run's curve, you get the Balanced PB.
/// The position of the Balanced PB on the x-axis is the PB chance.
#[derive(Default, Clone)]
pub struct SkillCurve {
    all_weighted_segment_times: Vec<Vec<(f64, TimeSpan)>>,
}

impl SkillCurve {
    /// Constructs a new empty skill curve. Before querying information, you
    /// need to calculate the curve for some segments.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the amount of segments this skill curve is comprised of.
    pub fn len(&self) -> usize {
        self.all_weighted_segment_times.len()
    }

    /// Returns `true` if there are no segments in this skill curve.
    pub fn is_empty(&self) -> bool {
        self.all_weighted_segment_times.is_empty()
    }

    /// Reduces the amount of segments that are being considered by this curve.
    pub fn truncate(&mut self, len: usize) {
        self.all_weighted_segment_times.truncate(len);
    }

    /// Calculate the skill curve for the segments and timing method provided.
    /// All previous information available in the skill curve will be discarded.
    /// The segment curve may not always contain information for all segments.
    /// Once a segment with no segment history is encountered, the remainder of
    /// the segments get discarded.
    pub fn for_segments(&mut self, segments: &[Segment], method: TimingMethod) {
        let mut len = segments.len();

        self.all_weighted_segment_times
            .resize_with(len, Default::default);

        for ((i, segment), weighted_segment_times) in segments
            .iter()
            .enumerate()
            .zip(&mut self.all_weighted_segment_times)
        {
            weighted_segment_times.clear();

            // Collect initial weighted segments
            let mut current_weight = 1.0;
            for &(id, time) in segment.segment_history().iter_actual_runs().rev() {
                if let Some(time) = time[method] {
                    // Skip all the combined segments
                    let skip = catch! {
                        segments[i.checked_sub(1)?].segment_history().get(id)?[method].is_none()
                    }
                    .unwrap_or(false);

                    if !skip {
                        weighted_segment_times.push((current_weight, time));
                        current_weight *= WEIGHT;
                    }
                }
            }

            // End early if we don't have any segment times anymore
            if weighted_segment_times.is_empty() {
                len = i;
                break;
            }

            // Sort everything by the times
            weighted_segment_times
                .sort_unstable_by_key(|&(_, time)| OrderedFloat(time.total_milliseconds()));

            // Cumulative sum of the weights
            let mut sum = 0.0;
            for (weight, _) in weighted_segment_times.iter_mut() {
                sum += *weight;
                *weight = sum;
            }

            // Reweigh all of the weights to be in the range 0..1
            let min = weighted_segment_times
                .first()
                .map(|&(w, _)| w)
                .unwrap_or_default();

            let max = weighted_segment_times
                .last()
                .map(|&(w, _)| w)
                .unwrap_or_default();

            let diff = max - min;

            if diff != 0.0 {
                for (weight, _) in weighted_segment_times.iter_mut() {
                    *weight = (*weight - min) / diff;
                }
            }
        }

        // Limit the slice to only the segments that have segment times.
        self.truncate(len);
    }

    /// This function returns an iterator that iterates over each segment and
    /// yields their segment times for the percentile specified. A percentile of
    /// 0 yields the fastest times, while a percentile of 1 yields the slowest
    /// times.
    pub fn iter_segment_times_at_percentile(
        &self,
        percentile: f64,
    ) -> impl Iterator<Item = TimeSpan> + '_ {
        self.all_weighted_segment_times
            .iter()
            .map(move |weighted_segment_times| {
                let found_index = weighted_segment_times
                    .binary_search_by(|&(w, _)| w.partial_cmp(&percentile).unwrap());

                match found_index {
                    // The percentile perfectly matched a segment time
                    Ok(index) => weighted_segment_times[index].1,
                    // The percentile didn't perfectly match, interpolate instead
                    Err(right_index) => {
                        // Saturate both left and right index at the bounds of the
                        // array. Both could go out of bounds.
                        let left_index = right_index.saturating_sub(1);
                        // This assumes len can never be 0. This only works out
                        // for us, because we truncate all the empty segments away.
                        let right_index = right_index.min(weighted_segment_times.len() - 1);

                        if left_index == right_index {
                            weighted_segment_times[left_index].1
                        } else {
                            let right = weighted_segment_times[right_index];
                            let left = weighted_segment_times[left_index];

                            interpolate(percentile, left, right)
                        }
                    }
                }
            })
    }

    /// This function returns an iterator that iterates over each segment and
    /// yields their split times for the percentile specified. A percentile of 0
    /// yields the fastest times, while a percentile of 1 yields the slowest
    /// times. The offset provided is the initial split time going into the
    /// first segment. This is only relevant when the segments don't represent
    /// the beginning of the run.
    pub fn iter_split_times_at_percentile(
        &self,
        percentile: f64,
        offset: TimeSpan,
    ) -> impl Iterator<Item = TimeSpan> + '_ {
        let mut sum = offset;
        self.iter_segment_times_at_percentile(percentile)
            .map(move |segment_time| {
                sum += segment_time;
                sum
            })
    }

    /// Searches the curve for the final run time specified and returns the
    /// percentile the time can be found at. The percentile is always within the
    /// range 0..1. If the time specified can not be found, the percentile
    /// saturates at one of those boundaries.
    pub fn find_percentile_for_time(&self, offset: TimeSpan, time_to_find: TimeSpan) -> f64 {
        let (mut perc_min, mut perc_max) = (0.0, 1.0);

        // Try to find the correct percentile
        for _ in 0..TRIES {
            let percentile = (perc_max + perc_min) / 2.0;

            let sum = self
                .iter_segment_times_at_percentile(percentile)
                .fold(offset, |sum, segment_time| sum + segment_time);

            // Binary search the correct percentile
            match sum.partial_cmp(&time_to_find) {
                Some(Ordering::Equal) => return percentile,
                Some(Ordering::Less) => perc_min = percentile,
                Some(Ordering::Greater) => perc_max = percentile,
                None => {}
            }
        }

        (perc_max + perc_min) / 2.0
    }
}

fn interpolate(
    perc: f64,
    (weight_left, time_left): (f64, TimeSpan),
    (weight_right, time_right): (f64, TimeSpan),
) -> TimeSpan {
    let perc_down =
        (weight_right - perc) * time_left.total_milliseconds() / (weight_right - weight_left);
    let perc_up =
        (perc - weight_left) * time_right.total_milliseconds() / (weight_right - weight_left);
    TimeSpan::from_milliseconds(perc_up + perc_down)
}
