//! Defines the Comparison Generator for calculating the Median Segments of a
//! Run. The Median Segments are calculated through a weighted median that gives
//! more recent segments a larger weight so that the Median Segments are more
//! suited to represent the current performance of a runner.

use super::ComparisonGenerator;
use ordered_float::OrderedFloat;
use {Attempt, Segment, TimeSpan, TimingMethod};

/// The Comparison Generator for calculating the Median Segments of a Run. The
/// Median Segments are calculated through a weighted median that gives more
/// recent segments a larger weight so that the Median Segments are more suited
/// to represent the current performance of a runner.
#[derive(Copy, Clone, Debug)]
pub struct MedianSegments;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Median";
/// The name of this comparison.
pub const NAME: &str = "Median Segments";

const WEIGHT: f64 = 0.75;

fn generate(segments: &mut [Segment], medians: &mut Vec<(f64, f64)>, method: TimingMethod) {
    let mut accumulated = Some(TimeSpan::zero());

    // TODO This may actually be possible to be fixed with a window like
    // iterator.
    #[allow(needless_range_loop)]
    for i in 0..segments.len() {
        // TODO Borrowcheck. if accumulated.is_some() is only necessary because
        // we can't assign to the outer variable otherwise.
        if accumulated.is_some() {
            medians.clear();

            let mut current_weight = 1.0;

            for &(id, time) in segments[i].segment_history().iter_actual_runs().rev() {
                if let Some(time) = time[method] {
                    // Skip all the combined segments
                    let skip = catch! {
                        segments[i.checked_sub(1)?].segment_history().get(id)?[method].is_none()
                    }.unwrap_or(false);

                    if !skip {
                        medians.push((current_weight, time.total_seconds()));
                        current_weight *= WEIGHT;
                    }
                }
            }

            if medians.is_empty() {
                accumulated = None;
            } else {
                medians.sort_unstable_by_key(|&(_, time)| OrderedFloat(time));
                let mut total_weights = 0.0;
                for (weight, _) in medians.iter_mut() {
                    *weight += total_weights;
                    total_weights = *weight;
                }
                let found_index = medians
                    .binary_search_by_key(&OrderedFloat(total_weights / 2.0), |&(weight, _)| {
                        OrderedFloat(weight)
                    });
                let segment_time = match found_index {
                    Ok(index) => medians[index].1,
                    Err(right_index) => medians[right_index].1,
                };
                accumulated = Some(accumulated.unwrap() + TimeSpan::from_seconds(segment_time));
            }
        }
        segments[i].comparison_mut(NAME)[method] = accumulated;
    }
}

impl ComparisonGenerator for MedianSegments {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        let medians = &mut Vec::new();
        generate(segments, medians, TimingMethod::RealTime);
        generate(segments, medians, TimingMethod::GameTime);
    }
}
