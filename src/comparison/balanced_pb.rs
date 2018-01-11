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

fn generate(segments: &mut [Segment], attempts: &[Attempt], method: TimingMethod) {
    // TODO Reuse. Actually it seems like this can be folded completely away
    // into the next loop, so we don't need a Vec at all (only the inner ones)
    // Actually, there's side effects between the segments in the first loop, so
    // we might not be able to do it after all.
    let mut all_history = vec![Vec::new(); segments.len()];

    for attempt in attempts {
        let attempt_index = attempt.index();
        let mut history_starting_index = -1;

        for (segment_index, (segment, all_history)) in
            segments.iter().zip(all_history.iter_mut()).enumerate()
        {
            let segment_index = segment_index as i64;

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

    // TODO Reuse
    let mut weighted_lists = Vec::new();
    let mut overall_starting_index = -1;

    for (current_index, (current_list, segment)) in
        all_history.into_iter().zip(segments.iter()).enumerate()
    {
        let mut null_segment = false;
        let current_pb_time = segment.personal_best_split_time()[method];
        let previous_pb_time = if overall_starting_index >= 0 {
            segments[overall_starting_index as usize].personal_best_split_time()[method]
        } else {
            None
        };

        // TODO More allocations :(
        let mut final_list = current_list
            .into_iter()
            .filter(|&(index, _)| index == overall_starting_index)
            .map(|(_, time)| time)
            .collect::<Vec<_>>();

        if !final_list.is_empty() {
            overall_starting_index = current_index as i64;
        } else if let Some(diff) = catch! { current_pb_time? - previous_pb_time? } {
            final_list.push(diff);
            overall_starting_index = current_index as i64;
        } else {
            null_segment = true;
        }

        if !null_segment {
            let count = final_list.len();
            // TODO More allocations FeelsBadMan at this point
            let mut temp_list = final_list
                .into_iter()
                .enumerate()
                .map(|(i, time)| (get_weight(i, count), time))
                .collect::<Vec<_>>();

            // TODO Aaarghhh >:(
            let mut weighted_list = Vec::new();

            if temp_list.len() > 1 {
                temp_list
                    .sort_unstable_by_key(|&(_, time)| OrderedFloat(time.total_milliseconds()));

                let total_weight = temp_list.iter().map(|&(weight, _)| weight).sum::<f64>();
                // TODO That's the smallest time's weight, not the actual
                // smallest weight. Is that intended?
                let smallest_weight = temp_list[0].0;
                let range_weight = total_weight - smallest_weight;

                let mut agg_weight = 0.0;
                for (weight, time) in temp_list {
                    agg_weight += weight;
                    // TODO Possibly use extend instead for better reallocation behavior
                    weighted_list.push((reweight(agg_weight, smallest_weight, range_weight), time));
                }

                // TODO What's the point of sorting this? temp_list was already
                // sorted by the times and was inserted in that order.
                // weighted_list was empty before.
                weighted_list
                    .sort_unstable_by_key(|&(_, time)| OrderedFloat(time.total_milliseconds()));
            } else {
                weighted_list.push((1.0, temp_list[0].1));
            }

            weighted_lists.push(Some(weighted_list));
        } else {
            weighted_lists.push(None);
        }
    }

    let goal_time = segments
        .last()
        .and_then(|s| s.personal_best_split_time()[method]);

    // TODO Damn, we actually reuse something this time. However even this list
    // may be completely unnecessary. Also it seems like these should be
    // optional. If we can't get rid of this, try to reuse it across both timing
    // methods.
    let mut output_splits = Vec::new();
    let (mut perc_min, mut perc_max) = (0.0, 1.0);
    let mut loop_protection = 0;

    loop {
        let mut run_sum = TimeSpan::zero();
        output_splits.clear();
        let percentile = 0.5 * (perc_max - perc_min) + perc_min;

        for weighted_list in &weighted_lists {
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
                output_splits.push(current_value);
                run_sum += current_value;
            } else {
                output_splits.push(TimeSpan::zero());
            }
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
    for (segment, &output_time) in segments.iter_mut().zip(output_splits.iter()) {
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
        generate(segments, attempts, TimingMethod::RealTime);
        generate(segments, attempts, TimingMethod::GameTime);
    }
}
