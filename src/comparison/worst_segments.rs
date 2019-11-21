//! Defines the Comparison Generator for calculating the Worst Segments of a Run.

use super::ComparisonGenerator;
use crate::analysis::sum_of_segments::worst::calculate;
use crate::platform::prelude::*;
use crate::{Attempt, Segment, Time, TimingMethod};

/// The Comparison Generator for calculating the Worst Segments of a Run.
#[derive(Copy, Clone, Debug)]
pub struct WorstSegments;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Worst";
/// The name of this comparison.
pub const NAME: &str = "Worst Segments";

impl ComparisonGenerator for WorstSegments {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        let mut predictions = Vec::with_capacity(segments.len() + 1);

        segments
            .iter_mut()
            .for_each(|s| *s.comparison_mut(NAME) = Time::new());

        for &method in &TimingMethod::all() {
            predictions.clear();
            predictions.resize(segments.len() + 1, None);

            calculate(segments, &mut predictions, false, method);

            let mut index = predictions
                .iter()
                .rposition(Option::is_some)
                .expect("There must always be a first sentinel prediction that is not None");
            while let Some(segment_index) = index.checked_sub(1) {
                let prediction =
                    predictions[index].expect("A predecessor prediction always needs to exist");
                segments[segment_index].comparison_mut(NAME)[method] = Some(prediction.time);
                index = prediction.predecessor;
            }
        }
    }
}
