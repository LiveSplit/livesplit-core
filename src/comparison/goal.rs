//! Defines functions for generating a goal comparison based on a goal time provided.
//! The comparison's times are automatically balanced based on the runner's
//! history such that it roughly represents what split times for the goal time
//! would roughly look like. This does not define a Comparison Generator. The
//! Balanced PB comparison however is based on this, which uses the Personal
//! Best as a goal time to balance the mistakes that happened in the Personal Best.

use crate::platform::prelude::*;
use crate::{Segment, Time, TimeSpan, TimingMethod};
use ordered_float::OrderedFloat;

/// The default name of the goal comparison.
pub const NAME: &str = "Goal";

const WEIGHT: f64 = 0.75;
const TRIES: usize = 50;

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

// FIXME: Possibly move this into the analysis module.
pub(crate) fn determine_percentile(
    offset: TimeSpan,
    segments: &[Segment],
    method: TimingMethod,
    goal_time: Option<TimeSpan>,
    time_span_buf: &mut Vec<TimeSpan>,
    all_weighted_segment_times: &mut [Vec<(f64, TimeSpan)>],
) -> f64 {
    let mut len = segments.len();

    for ((i, segment), weighted_segment_times) in segments
        .iter()
        .enumerate()
        .zip(all_weighted_segment_times.iter_mut())
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
    let mut all_weighted_segment_times = &mut all_weighted_segment_times[..len];

    // Depending on whether we have a goal time or not, we used that goal time
    // or try to determine a personal best split time that we use for the goal
    // time. In that case we may need to limit the slice again to the last split
    // that actually has a split time we can work with.
    let goal_time = if let Some(goal_time) = goal_time {
        goal_time
    } else {
        let (new_len, goal_time) = segments[..len]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, s)| s.personal_best_split_time()[method].map(|t| (i + 1, t)))
            .unwrap_or_default();
        all_weighted_segment_times = &mut all_weighted_segment_times[..new_len];
        goal_time
    };

    let (mut perc_min, mut perc_max) = (0.0, 1.0);

    // Try to find the correct percentile
    for _ in 0..TRIES {
        let percentile = (perc_max + perc_min) / 2.0;
        let mut sum = offset;

        time_span_buf.clear();
        time_span_buf.extend(
            all_weighted_segment_times
                .iter()
                .map(|weighted_segment_times| {
                    // Binary search the percentile in the segment's segment times
                    let percentile_segment_time = if weighted_segment_times.len() == 1 {
                        // Shortcut for a single segment time
                        weighted_segment_times[0].1
                    } else {
                        let found_index = weighted_segment_times
                            .binary_search_by(|&(w, _)| w.partial_cmp(&percentile).unwrap());

                        match found_index {
                            // The percentile perfectly matched a segment time
                            Ok(index) => weighted_segment_times[index].1,
                            // The percentile didn't perfectly match, interpolate instead
                            Err(right_index) => {
                                let right = weighted_segment_times[right_index];
                                let left = right_index
                                    .checked_sub(1)
                                    .map(|left_index| weighted_segment_times[left_index])
                                    .unwrap_or_default();

                                interpolate(percentile, left, right)
                            }
                        }
                    };

                    sum += percentile_segment_time;
                    sum
                }),
        );

        // Binary search the correct percentile
        if sum == goal_time {
            return percentile;
        } else if sum < goal_time {
            perc_min = percentile;
        } else {
            perc_max = percentile;
        }
    }

    (perc_max + perc_min) / 2.0
}

pub(super) fn generate_for_timing_method_with_buf(
    segments: &mut [Segment],
    method: TimingMethod,
    goal_time: Option<TimeSpan>,
    comparison: &str,
    time_span_buf: &mut Vec<TimeSpan>,
    all_weighted_segment_times: &mut [Vec<(f64, TimeSpan)>],
) {
    let _percentile = determine_percentile(
        TimeSpan::zero(),
        segments,
        method,
        goal_time,
        time_span_buf,
        all_weighted_segment_times,
    );

    for (segment, &val) in segments.iter_mut().zip(time_span_buf.iter()) {
        segment.comparison_mut(comparison)[method] = Some(val);
    }
    for segment in &mut segments[time_span_buf.len()..] {
        segment.comparison_mut(comparison)[method] = None;
    }
}

/// Populates the segments with a goal comparison for the timing method
/// specified. Every other timing method is left untouched. The segment history
/// is used to generate comparison times such that they end up with the goal
/// time specified. The values are stored in the comparison with the name
/// provided. Only the range between the sum of the best segments and the sum of
/// the worst segments is supported. Every other goal time is capped within that
/// range.
pub fn generate_for_timing_method(
    segments: &mut [Segment],
    method: TimingMethod,
    goal_time: TimeSpan,
    comparison: &str,
) {
    let mut all_weighted_segment_times = vec![Vec::new(); segments.len()];
    let mut time_span_buf = Vec::with_capacity(segments.len());

    generate_for_timing_method_with_buf(
        segments,
        method,
        Some(goal_time),
        comparison,
        &mut time_span_buf,
        &mut all_weighted_segment_times,
    );
}

/// Populates the segments with a goal comparison. The segment history is used
/// to generate comparison times such that they end up with the goal time
/// specified. The values are stored in the comparison with the name provided.
/// Only the range between the sum of the best segments and the sum of the worst
/// segments is supported. Every other goal time is capped within that range.
pub fn generate(segments: &mut [Segment], goal_time: Time, comparison: &str) {
    let mut all_weighted_segment_times = vec![Vec::new(); segments.len()];
    let mut time_span_buf = Vec::with_capacity(segments.len());

    if let Some(real_time) = goal_time.real_time {
        generate_for_timing_method_with_buf(
            segments,
            TimingMethod::RealTime,
            Some(real_time),
            comparison,
            &mut time_span_buf,
            &mut all_weighted_segment_times,
        );
    } else {
        for segment in &mut *segments {
            segment.comparison_mut(comparison).real_time = None;
        }
    }

    if let Some(game_time) = goal_time.game_time {
        generate_for_timing_method_with_buf(
            segments,
            TimingMethod::GameTime,
            Some(game_time),
            comparison,
            &mut time_span_buf,
            &mut all_weighted_segment_times,
        );
    } else {
        for segment in &mut *segments {
            segment.comparison_mut(comparison).game_time = None;
        }
    }
}
