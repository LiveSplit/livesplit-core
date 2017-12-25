//! Provides functionality for calculating the Sum of Best Segments for whole
//! runs or specific parts. The Sum of Best Segments is the fastest time
//! possible to complete a run of a category, based on information collected
//! from all the previous attempts. This often matches up with the sum of the
//! best segment times of all the segments, but that may not always be the case,
//! as skipped segments may introduce combined segments that may be faster than
//! the actual sum of their best segment times. The name is therefore a bit
//! misleading, but sticks around for historical reasons.

use {Segment, TimeSpan, TimingMethod};
use super::{track_branch, track_current_run, track_personal_best_run};

fn populate_prediction(prediction: &mut Option<TimeSpan>, predicted_time: Option<TimeSpan>) {
    if let Some(predicted_time) = predicted_time {
        if prediction.map_or(true, |t| predicted_time < t) {
            *prediction = Some(predicted_time);
        }
    }
}

fn populate_predictions(
    segments: &[Segment],
    current_time: Option<TimeSpan>,
    segment_index: usize,
    predictions: &mut [Option<TimeSpan>],
    simple_calculation: bool,
    use_current_run: bool,
    method: TimingMethod,
) {
    if let Some(current_time) = current_time {
        populate_prediction(
            &mut predictions[segment_index + 1],
            segments[segment_index].best_segment_time()[method].map(|t| t + current_time),
        );
        if !simple_calculation {
            for &(null_segment_index, _) in segments[segment_index]
                .segment_history()
                .iter()
                .filter(|&&(_, t)| t[method].is_none())
            {
                let should_track_branch = segment_index
                    .checked_sub(1)
                    .and_then(|previous_index| {
                        segments[previous_index]
                            .segment_history()
                            .get(null_segment_index)
                    })
                    .map_or(true, |segment_time| segment_time[method].is_some());

                if should_track_branch {
                    let (index, time) = track_branch(
                        segments,
                        Some(current_time),
                        segment_index + 1,
                        null_segment_index,
                        method,
                    );
                    populate_prediction(&mut predictions[index], time[method]);
                }
            }
        }
        if use_current_run {
            let (index, time) =
                track_current_run(segments, Some(current_time), segment_index, method);
            populate_prediction(&mut predictions[index], time[method]);
        }
        let (index, time) =
            track_personal_best_run(segments, Some(current_time), segment_index, method);
        populate_prediction(&mut predictions[index], time[method]);
    }
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
/// choose to take it into account as well. This lower level function requires
/// you to provide a buffer to fill up with the shortest paths to reach each of
/// the segments.
#[allow(needless_range_loop, unknown_lints)]
pub fn calculate(
    segments: &[Segment],
    start_index: usize,
    end_index: usize, // Exclusive
    predictions: &mut [Option<TimeSpan>],
    simple_calculation: bool,
    use_current_run: bool,
    method: TimingMethod,
) -> Option<TimeSpan> {
    predictions[start_index] = Some(TimeSpan::zero());
    for segment_index in start_index..end_index {
        let current_time = predictions[segment_index];
        populate_predictions(
            segments,
            current_time,
            segment_index,
            predictions,
            simple_calculation,
            use_current_run,
            method,
        );
    }
    predictions[end_index]
}
