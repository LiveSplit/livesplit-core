//! Defines the Comparison Generator for calculating the Best Segments of a
//! [`Run`](crate::Run).

use super::ComparisonGenerator;
use crate::{
    Attempt, Segment, Time, TimeSpan, TimingMethod, analysis::sum_of_segments::calculate_best,
};

/// Defines the Comparison Generator for calculating the Best Segments of a
/// [`Run`](crate::Run).
#[derive(Copy, Clone, Debug)]
pub struct MergedTimeloss;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Merged";
/// The name of this comparison.
pub const NAME: &str = "Merged Timeloss";

impl ComparisonGenerator for MergedTimeloss {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        segments
            .iter_mut()
            .for_each(|s| *s.comparison_mut(NAME) = Time::new());

        for method in TimingMethod::all() {
            let mut split_time = calculate_timeloss(&segments, method);

            for segment in &mut *segments {
                let mut time = None;
                if let Some(segment_best) = segment.best_segment_time()[method] {
                    split_time = segment_best + split_time;
                    time = Some(split_time)
                }
                segment.comparison_mut(NAME)[method] = time;
            }
        }
    }
}
fn calculate_timeloss(segments: &[Segment], method: TimingMethod) -> TimeSpan {
    if segments.len() == 0 { return TimeSpan::zero() }
    let pb = segments[segments.len()-1].personal_best_split_time()[method];
    let best = calculate_best(segments, false, false, method);

    if pb.is_some() && best.is_some()
    {
        pb.unwrap() - best.unwrap()
    } else { TimeSpan::zero() }
}
