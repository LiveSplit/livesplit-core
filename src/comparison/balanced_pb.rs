use super::ComparisonGenerator;
use {Attempt, Segment, TimeSpan, TimingMethod};
use ordered_float::OrderedFloat;

#[derive(Copy, Clone, Debug)]
pub struct BalancedPB;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Balanced";
/// The name of this comparison.
pub const NAME: &str = "Balanced PB";

const WEIGHT: f64 = 0.9375;

fn get_weight(index: usize, count: usize) -> f64 {
    WEIGHT.powi((count - index - 1) as i32)
}

fn reweight(a: f64, b: f64, c: f64) -> f64 {
    (a - b) / c
}

fn calculate(
    perc: f64,
    (key1, value1): (f64, TimeSpan),
    (key2, value2): (f64, TimeSpan),
) -> TimeSpan {
    let perc_down = (key1 - perc) * value2.total_milliseconds() / (key1 - key2);
    let perc_up = (perc - key2) * value1.total_milliseconds() / (key1 - key2);
    TimeSpan::from_milliseconds(perc_up + perc_down)
}

fn generate(
    segments: &mut [Segment],
    attempts: &[Attempt],
    method: TimingMethod,
    all_history: &mut [Vec<(isize, TimeSpan)>],
    weighted_lists: &mut [Option<Vec<(f64, TimeSpan)>>],
    time_span_buf: &mut Vec<TimeSpan>,
) {
    for attempt in attempts {
        let attempt_index = attempt.index();
        let mut history_starting_index = -1;

        for (segment_index, (segment, all_history)) in
            segments.iter().zip(all_history.iter_mut()).enumerate()
        {
            let segment_index = segment_index as isize;

            if let Some(history) = segment.segment_history().get(attempt_index) {
                if let Some(history) = history[method] {
                    all_history.push((history_starting_index, history));
                    history_starting_index = segment_index;
                }
            } else {
                history_starting_index = segment_index;
            }
        }
    }

    let mut overall_starting_index = -1;

    for (current_index, ((current_list, segment), weighted_list_slot)) in all_history
        .iter_mut()
        .zip(segments.iter())
        .zip(weighted_lists.iter_mut())
        .enumerate()
    {
        let mut null_segment = false;
        let current_pb_time = segment.personal_best_split_time()[method];
        let previous_pb_time = if overall_starting_index >= 0 {
            segments[overall_starting_index as usize].personal_best_split_time()[method]
        } else {
            None
        };

        time_span_buf.clear();
        time_span_buf.extend(
            current_list
                .drain(..)
                .filter(|&(index, _)| index == overall_starting_index)
                .map(|(_, time)| time),
        );

        if !time_span_buf.is_empty() {
            overall_starting_index = current_index as isize;
        } else if let Some(diff) = catch! { current_pb_time? - previous_pb_time? } {
            time_span_buf.push(diff);
            overall_starting_index = current_index as isize;
        } else {
            null_segment = true;
        }

        if !null_segment {
            let count = time_span_buf.len();

            // This reuses the Vec from the previous timing method if possible.
            let weighted_list = weighted_list_slot.get_or_insert_with(Vec::new);
            weighted_list.clear();
            weighted_list.extend(
                time_span_buf
                    .drain(..)
                    .enumerate()
                    .map(|(i, time)| (get_weight(i, count), time)),
            );

            if weighted_list.len() > 1 {
                weighted_list
                    .sort_unstable_by_key(|&(_, time)| OrderedFloat(time.total_milliseconds()));

                let total_weight = weighted_list.iter().map(|&(weight, _)| weight).sum::<f64>();
                // TODO That's the smallest time's weight, not the actual
                // smallest weight. Is that intended?
                let smallest_weight = weighted_list[0].0;
                let range_weight = total_weight - smallest_weight;

                let mut agg_weight = 0.0;
                for &mut (ref mut weight, _) in weighted_list.iter_mut() {
                    agg_weight += *weight;
                    *weight = reweight(agg_weight, smallest_weight, range_weight);
                }

            // TODO What's the point of sorting this? temp_list was already
            // sorted by the times and was inserted in that order.
            // weighted_list was empty before.
            // weighted_list
            //     .sort_unstable_by_key(|&(_, time)| OrderedFloat(time.total_milliseconds()));
            } else {
                weighted_list[0].0 = 1.0;
            }
        } else {
            *weighted_list_slot = None;
        }
    }

    let goal_time = segments
        .last()
        .and_then(|s| s.personal_best_split_time()[method]);

    let (mut perc_min, mut perc_max) = (0.0, 1.0);
    let mut loop_protection = 0;

    loop {
        let mut run_sum = TimeSpan::zero();
        let percentile = 0.5 * (perc_max - perc_min) + perc_min;

        time_span_buf.clear();
        for weighted_list in weighted_lists.iter() {
            if let &Some(ref weighted_list) = weighted_list {
                let mut current_value = TimeSpan::zero();
                if weighted_list.len() > 1 {
                    for (n, &(weight, time)) in weighted_list.iter().enumerate() {
                        if weight > percentile {
                            current_value =
                                calculate(percentile, (weight, time), weighted_list[n - 1]);
                            break;
                        }
                        if weight == percentile {
                            current_value = time;
                            break;
                        }
                    }
                } else {
                    current_value = weighted_list[0].1;
                }
                run_sum += current_value;
                time_span_buf.push(current_value);
            } else {
                time_span_buf.push(TimeSpan::zero());
            };
        }

        if let Some(goal_time) = goal_time {
            if run_sum > goal_time {
                perc_max = percentile;
            } else {
                perc_min = percentile;
            }
            loop_protection += 1;

            if run_sum == goal_time || loop_protection >= 50 {
                break;
            }
        } else {
            break;
        }
    }

    let mut total_time = TimeSpan::zero();
    for (segment, &output_time) in segments.iter_mut().zip(time_span_buf.iter()) {
        total_time += output_time;
        segment.comparison_mut(NAME)[method] = if output_time == TimeSpan::zero() {
            None
        } else {
            Some(total_time)
        };
    }
}

impl ComparisonGenerator for BalancedPB {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], attempts: &[Attempt]) {
        let mut all_history = vec![Vec::new(); segments.len()];
        let mut weighted_lists = vec![None; segments.len()];
        let mut time_span_buf = Vec::with_capacity(segments.len());

        generate(
            segments,
            attempts,
            TimingMethod::RealTime,
            &mut all_history,
            &mut weighted_lists,
            &mut time_span_buf,
        );
        generate(
            segments,
            attempts,
            TimingMethod::GameTime,
            &mut all_history,
            &mut weighted_lists,
            &mut time_span_buf,
        );
    }
}
