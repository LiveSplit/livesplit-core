//! Defines the Comparison Generator for calculating the Latest Run. Using the
//! Segment History, this comparison reconstructs the splits of the furthest,
//! most recent attempt. If at least one attempt has been finished, this
//! comparison will show the most recent finished attempt. If no attempts have
//! been finished yet, this comparison will show the attempt that got the
//! furthest.

use super::ComparisonGenerator;
use crate::{Attempt, Segment, TimeSpan, TimingMethod};

/// The Comparison Generator for calculating the Latest Run. Using the
/// Segment History, this comparison reconstructs the splits of the furthest,
/// most recent attempt. If at least one attempt has been finished, this
/// comparison will show the most recent finished attempt. If no attempts have
/// been finished yet, this comparison will show the attempt that got the
/// furthest.
#[derive(Copy, Clone, Debug)]
pub struct LatestRun;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Latest";
/// The name of this comparison.
pub const NAME: &str = "Latest Run";

fn generate(segments: &mut [Segment], method: TimingMethod) {
    let mut attempt_id = None;
    for segment in segments.iter_mut().rev() {
        if let Some(max_index) = segment.segment_history().try_get_max_index() {
            attempt_id = Some(max_index);
            break;
        }
    }

    if let Some(attempt_id) = attempt_id {
        let mut remaining_segments = segments.iter_mut();

        let mut total_time = TimeSpan::zero();
        for segment in remaining_segments.by_ref() {
            let segment_time = segment.segment_history().get(attempt_id).map(|t| t[method]);

            let split_time = match segment_time {
                Some(Some(segment_time)) => {
                    total_time += segment_time;
                    Some(total_time)
                }
                Some(None) => None,
                None => {
                    segment.comparison_mut(NAME)[method] = None;
                    break;
                }
            };

            segment.comparison_mut(NAME)[method] = split_time;
        }

        for segment in remaining_segments {
            segment.comparison_mut(NAME)[method] = None;
        }
    }
}

impl ComparisonGenerator for LatestRun {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        generate(segments, TimingMethod::RealTime);
        generate(segments, TimingMethod::GameTime);
    }
}
