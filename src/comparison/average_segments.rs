//! Defines the Comparison Generator for calculating the Average Segments of a
//! Run. The Average Segments are calculated through a weighted arithmetic mean
//! that gives more recent segments a larger weight so that the Average
//! Segments are more suited to represent the current performance of a
//! runner.

use super::ComparisonGenerator;
use crate::{Attempt, Segment, TimeSpan, TimingMethod};

/// The Comparison Generator for calculating the Average Segments of a Run. The
/// Average Segments are calculated through a weighted arithmetic mean that
/// gives more recent segments a larger weight so that the Average Segments are
/// more suited to represent the current performance of a runner.
#[derive(Copy, Clone, Debug)]
pub struct AverageSegments;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Average";
/// The name of this comparison.
pub const NAME: &str = "Average Segments";

const WEIGHT: f64 = 0.75;

fn generate(segments: &mut [Segment], method: TimingMethod) {
    let mut accumulated = Some(TimeSpan::zero());

    let mut previous_segment: Option<&Segment> = None;
    for segment in segments {
        if let Some(accumulated_val) = &mut accumulated {
            let (mut total_weights, mut total_time) = (0.0, 0.0);
            let mut current_weight = 1.0;

            for &(id, time) in segment.segment_history().iter_actual_runs().rev() {
                if let Some(time) = time[method] {
                    // Skip all the combined segments
                    let skip = catch! {
                        previous_segment?.segment_history().get(id)?[method].is_none()
                    }
                    .unwrap_or(false);

                    if !skip {
                        total_weights += current_weight;
                        total_time += current_weight * time.total_seconds();
                        current_weight *= WEIGHT;
                    }
                }
            }

            if total_weights == 0.0 {
                accumulated = None;
            } else {
                *accumulated_val += TimeSpan::from_seconds(total_time / total_weights);
            }
        }
        segment.comparison_mut(NAME)[method] = accumulated;
        previous_segment = Some(&*segment);
    }
}

impl ComparisonGenerator for AverageSegments {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        generate(segments, TimingMethod::RealTime);
        generate(segments, TimingMethod::GameTime);
    }
}
