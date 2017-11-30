//! Defines the Comparison Generator for the Best Split Times. The Best Split
//! Times are the fastest actual split times a runner got for each individual
//! split. This differs from the Personal Best, as the Personal Best only
//! represents the fastest split time for the final split, but other runs that
//! may have had major mistakes later on and possibly may have been reset, could
//! have had a better pace for earlier splits.

use super::ComparisonGenerator;
use {Attempt, Segment, TimeSpan, TimingMethod};

/// The Comparison Generator for the Best Split Times. The Best Split
/// Times are the fastest actual split times a runner got for each individual
/// split. This differs from the Personal Best, as the Personal Best only
/// represents the fastest split time for the final split, but other runs that
/// may have had major mistakes later on and possibly may have been reset, could
/// have had a better pace for earlier splits.
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
