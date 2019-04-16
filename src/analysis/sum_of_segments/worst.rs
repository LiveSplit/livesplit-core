//! Provides functionality for calculating the Sum of Worst Segments for whole
//! runs or specific parts. The Sum of Worst Segments is the slowest time
//! possible to complete a run of a category, based on information collected
//! from all the previous attempts. This obviously isn't really the worst
//! possible time, but may be useful information regardless.

use super::{track_branch, track_current_run, track_personal_best_run, Prediction};
use crate::{Segment, TimeSpan, TimingMethod};

fn populate_prediction(
    predecessor: usize,
    target_prediction: &mut Option<Prediction>,
    predicted_time: Option<TimeSpan>,
) {
    if let Some(predicted_time) = predicted_time {
        if target_prediction.map_or(true, |p| predicted_time > p.time) {
            *target_prediction = Some(Prediction {
                time: predicted_time,
                predecessor,
            });
        }
    }
}

fn populate_predictions(
    segments: &[Segment],
    current_prediction: Option<Prediction>,
    segment_index: usize,
    predictions: &mut [Option<Prediction>],
    use_current_run: bool,
    method: TimingMethod,
) {
    if let Some(Prediction {
        time: current_time, ..
    }) = current_prediction
    {
        populate_prediction(
            segment_index,
            &mut predictions[segment_index + 1],
            segments[segment_index].best_segment_time()[method].map(|t| t + current_time),
        );
        for &(segment_history_index, _) in segments[segment_index].segment_history().iter() {
            let should_track_branch = catch! {
                segments[segment_index.checked_sub(1)?]
                    .segment_history()
                    .get(segment_history_index)?[method]
                    .is_some()
            }
            .unwrap_or(true);

            if should_track_branch {
                let (index, time) = track_branch(
                    segments,
                    Some(current_time),
                    segment_index,
                    segment_history_index,
                    method,
                );
                populate_prediction(segment_index, &mut predictions[index], time[method]);
            }
        }
        if use_current_run {
            let (index, time) =
                track_current_run(segments, Some(current_time), segment_index, method);
            populate_prediction(segment_index, &mut predictions[index], time[method]);
        }
        let (index, time) =
            track_personal_best_run(segments, Some(current_time), segment_index, method);
        populate_prediction(segment_index, &mut predictions[index], time[method]);
    }
}

/// Calculates the Sum of Worst Segments for the timing method provided. This is
/// the slowest time possible to complete a run of a category, based on
/// information collected from all the previous attempts. This obviously isn't
/// really the worst possible time, but may be useful information regardless. If
/// there's an active attempt, you can choose to take it into account as well.
/// This lower level function requires you to provide a buffer to fill up with
/// the slowest paths to reach each of the segments. This means that the first
/// segment will always be reached at a time of 0:00. However, if you are
/// interested in the total Sum of Worst Segments, then you can't look at the
/// predictions value with the index of the last segment, as that only tells you
/// what the worst time to reach that segment is, not the worst time to complete
/// it. This means that the predictions buffer needs to have one more element
/// than the list of segments provided, so that you can properly query the total
/// Sum of Worst Segments. This value is also the value that is being returned.
#[allow(clippy::needless_range_loop)]
pub fn calculate(
    segments: &[Segment],
    predictions: &mut [Option<Prediction>],
    use_current_run: bool,
    method: TimingMethod,
) -> Option<TimeSpan> {
    predictions[0] = Some(Prediction::default());
    let end_index = segments.len();
    for segment_index in 0..end_index {
        populate_predictions(
            segments,
            predictions[segment_index],
            segment_index,
            predictions,
            use_current_run,
            method,
        );
    }
    Some(predictions[end_index]?.time)
}
