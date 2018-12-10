//! Defines the Comparison Generator for the Best Split Times. The Best Split
//! Times represent the best pace that the runner was ever on up to each split
//! in the run. The Best Split Times are calculated by taking the best split
//! time for each individual split from all of the runner's attempts.

use super::ComparisonGenerator;
use crate::{Attempt, Segment, TimeSpan, TimingMethod};

/// The Comparison Generator for the Best Split Times. The Best Split Times
/// represent the best pace that the runner was ever on up to each split in the
/// run. The Best Split Times are calculated by taking the best split time for
/// each individual split from all of the runner's attempts.
#[derive(Copy, Clone, Debug)]
pub struct BestSplitTimes;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = NAME;
/// The name of this comparison.
pub const NAME: &str = "Best Split Times";

fn generate(segments: &mut [Segment], attempts: &[Attempt], method: TimingMethod) {
    for attempt in attempts {
        let id = attempt.index();
        let mut total_time = TimeSpan::zero();

        for segment in segments.iter_mut() {
            if let Some(time) = segment.segment_history().get(id) {
                if let Some(time) = time[method] {
                    total_time += time;

                    let comp = &mut segment.comparison_mut(NAME)[method];
                    if comp.map_or(true, |c| total_time < c) {
                        *comp = Some(total_time);
                    }
                }
            } else {
                break;
            }
        }
    }
}

impl ComparisonGenerator for BestSplitTimes {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], attempts: &[Attempt]) {
        if !segments.is_empty() {
            *segments[0].comparison_mut(NAME) = segments[0].best_segment_time();
            for segment in &mut segments[1..] {
                *segment.comparison_mut(NAME) = segment.personal_best_split_time();
            }

            generate(segments, attempts, TimingMethod::RealTime);
            generate(segments, attempts, TimingMethod::GameTime);
        }
    }
}
