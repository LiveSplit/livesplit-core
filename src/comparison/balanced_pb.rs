//! Defines the Comparison Generator for calculating a comparison which has the
//! same final time as the runner's Personal Best. Unlike the Personal Best
//! however, all the other split times are automatically balanced by the
//! runner's history in order to balance out the mistakes present in the
//! Personal Best throughout the comparison. Running against an unbalanced
//! Personal Best can cause frustrations. A Personal Best with a mediocre early
//! game and a really good end game has a high chance of the runner losing a lot
//! of time compared to the Personal Best towards the end of a run. This may
//! discourage the runner, which may lead them to reset the attempt. That's the
//! perfect situation to compare against the Balanced Personal Best comparison
//! instead, as all of the mistakes of the early game in such a situation would
//! be smoothed out throughout the whole comparison.

use super::ComparisonGenerator;
use {Attempt, Segment, TimeSpan, TimingMethod};
use ordered_float::OrderedFloat;

/// The Comparison Generator for calculating a comparison which has the same
/// final time as the runner's Personal Best. Unlike the Personal Best however,
/// all the other split times are automatically balanced by the runner's history
/// in order to balance out the mistakes present in the Personal Best throughout
/// the comparison. Running against an unbalanced Personal Best can cause
/// frustrations. A Personal Best with a mediocre early game and a really good
/// end game has a high chance of the runner losing a lot of time compared to
/// the Personal Best towards the end of a run. This may discourage the runner,
/// which may lead them to reset the attempt. That's the perfect situation to
/// compare against the Balanced Personal Best comparison instead, as all of the
/// mistakes of the early game in such a situation would be smoothed out
/// throughout the whole comparison.
#[derive(Copy, Clone, Debug)]
pub struct BalancedPB;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Balanced";
/// The name of this comparison.
pub const NAME: &str = "Balanced PB";

const WEIGHT: f64 = 0.9375;

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

fn generate(
    segments: &mut [Segment],
    method: TimingMethod,
    time_span_buf: &mut Vec<TimeSpan>,
    all_weighted_segment_times: &mut [Vec<(f64, TimeSpan)>],
) {
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
                }.unwrap_or(false);

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
        for &mut (ref mut weight, _) in weighted_segment_times.iter_mut() {
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
            for &mut (ref mut weight, _) in weighted_segment_times.iter_mut() {
                *weight = (*weight - min) / diff;
            }
        }
    }

    // Limit the slice to only the segments that have segment times.
    let all_weighted_segment_times = &mut all_weighted_segment_times[..len];
    // Limit the slice again to the last split that actually has a split time we can work with.
    let (new_len, goal_time) = segments[..len]
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(i, s)| s.personal_best_split_time()[method].map(|t| (i + 1, t)))
        .next()
        .unwrap_or_default();
    let all_weighted_segment_times = &mut all_weighted_segment_times[..new_len];

    let (mut perc_min, mut perc_max) = (0.0, 1.0);

    // Try to find the correct percentile
    for _ in 0..50 {
        let percentile = (perc_max + perc_min) / 2.0;
        let mut sum = TimeSpan::zero();

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
            break;
        } else if sum < goal_time {
            perc_min = percentile;
        } else {
            perc_max = percentile;
        }
    }

    for (segment, &val) in segments.iter_mut().zip(time_span_buf.iter()) {
        segment.comparison_mut(NAME)[method] = Some(val);
    }
    for segment in &mut segments[time_span_buf.len()..] {
        segment.comparison_mut(NAME)[method] = None;
    }
}

impl ComparisonGenerator for BalancedPB {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        let mut all_weighted_segment_times = vec![Vec::new(); segments.len()];
        let mut time_span_buf = Vec::with_capacity(segments.len());

        generate(
            segments,
            TimingMethod::RealTime,
            &mut time_span_buf,
            &mut all_weighted_segment_times,
        );
        generate(
            segments,
            TimingMethod::GameTime,
            &mut time_span_buf,
            &mut all_weighted_segment_times,
        );
    }
}
