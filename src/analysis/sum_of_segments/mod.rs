//! Provides functionality for calculating the Sum of Best Segments and the Sum
//! of Worst Segments for whole runs or specific parts. The Sum of Best Segments
//! is the fastest time possible to complete a run of a category, based on
//! information collected from all the previous attempts. This often matches up
//! with the sum of the best segment times of all the segments, but that may not
//! always be the case, as skipped segments may introduce combined segments that
//! may be faster than the actual sum of their best segment times. The name is
//! therefore a bit misleading, but sticks around for historical reasons.

pub mod best;
pub mod worst;

#[cfg(test)]
mod tests;

use crate::platform::prelude::*;
use crate::{Segment, Time, TimeSpan, TimingMethod};

/// Describes the shortest amount of time it takes to reach a certain segment.
/// Since there is the possibility that the shortest path is actually skipping
/// segments, there's an additional predecessor index that describes the segment
/// this prediction is based on. By following all the predecessors backwards,
/// you can get access to the single fastest route.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Prediction {
    /// The shortest amount of time it takes to reach the segment.
    pub time: TimeSpan,
    /// The index of the predecessor that directly leads to this segment.
    pub predecessor: usize,
}

/// Calculates the Sum of Best Segments for the timing method provided. This is
/// the fastest time possible to complete a run of a category, based on
/// information collected from all the previous attempts. This often matches up
/// with the sum of the best segment times of all the segments, but that may not
/// always be the case, as skipped segments may introduce combined segments that
/// may be faster than the actual sum of their best segment times. The name is
/// therefore a bit misleading, but sticks around for historical reasons. You
/// can choose to do a simple calculation instead, which excludes the Segment
/// History from the calculation process. If there's an active attempt, you can
/// choose to take it into account as well.
pub fn calculate_best(
    segments: &[Segment],
    simple_calculation: bool,
    use_current_run: bool,
    method: TimingMethod,
) -> Option<TimeSpan> {
    let mut predictions = vec![None; segments.len() + 1];
    best::calculate(
        segments,
        &mut predictions,
        simple_calculation,
        use_current_run,
        method,
    )
}

/// Calculates the Sum of Worst Segments for the timing method provided. This is
/// the slowest time possible to complete a run of a category, based on
/// information collected from all the previous attempts. This obviously isn't
/// really the worst possible time, but may be useful information regardless.
/// If there's an active attempt, you can choose to take it into account as
/// well.
pub fn calculate_worst(
    segments: &[Segment],
    use_current_run: bool,
    method: TimingMethod,
) -> Option<TimeSpan> {
    let mut predictions = vec![None; segments.len() + 1];
    worst::calculate(segments, &mut predictions, use_current_run, method)
}

fn track_current_run(
    segments: &[Segment],
    current_time: Option<TimeSpan>,
    segment_index: usize,
    method: TimingMethod,
) -> (usize, Time) {
    if let Some(first_split_time) = segment_index
        .checked_sub(1)
        .map_or(Some(TimeSpan::zero()), |i| segments[i].split_time()[method])
    {
        for (segment_index, segment) in segments.iter().enumerate().skip(segment_index) {
            let second_split_time = segment.split_time()[method];
            if let Some(second_split_time) = second_split_time {
                return (
                    segment_index + 1,
                    Time::new().with_timing_method(
                        method,
                        current_time.map(|t| second_split_time - first_split_time + t),
                    ),
                );
            }
        }
    }
    (0, Time::default())
}

fn track_personal_best_run(
    segments: &[Segment],
    current_time: Option<TimeSpan>,
    segment_index: usize,
    method: TimingMethod,
) -> (usize, Time) {
    if let Some(first_split_time) = segment_index
        .checked_sub(1)
        .map_or(Some(TimeSpan::zero()), |i| {
            segments[i].personal_best_split_time()[method]
        })
    {
        for (segment_index, segment) in segments.iter().enumerate().skip(segment_index) {
            let second_split_time = segment.personal_best_split_time()[method];
            if let Some(second_split_time) = second_split_time {
                return (
                    segment_index + 1,
                    Time::new().with_timing_method(
                        method,
                        current_time.map(|t| second_split_time - first_split_time + t),
                    ),
                );
            }
        }
    }
    (0, Time::default())
}

/// Follows a path starting from a certain segment in a certain attempt to the
/// next split that didn't get skipped. Returns the index of the segment after
/// the segment that has the next split time and a sum of the combined segment
/// times and the current time provided. If the tracked attempt ends before a
/// split time is found, the index returned is 0.
pub fn track_branch(
    segments: &[Segment],
    current_time: Option<TimeSpan>,
    segment_index: usize,
    run_index: i32,
    method: TimingMethod,
) -> (usize, Time) {
    for (segment_index, segment) in segments.iter().enumerate().skip(segment_index) {
        if let Some(cur_time) = segment.segment_history().get(run_index) {
            if let Some(cur_time) = cur_time[method] {
                return (
                    segment_index + 1,
                    Time::new().with_timing_method(method, current_time.map(|t| cur_time + t)),
                );
            }
        } else {
            break;
        }
    }
    (0, Time::default())
}
