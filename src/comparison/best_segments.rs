//! Defines the Comparison Generator for calculating the Best Segments of a Run.

use super::ComparisonGenerator;
use {Attempt, Segment, Time, TimingMethod};
use analysis::sum_of_segments::best::calculate;

/// The Comparison Generator for calculating the Best Segments of a Run.
#[derive(Copy, Clone, Debug)]
pub struct BestSegments;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Best";
/// The name of this comparison.
pub const NAME: &str = "Best Segments";

impl ComparisonGenerator for BestSegments {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        let mut real_time_predictions = vec![None; segments.len() + 1];
        let mut game_time_predictions = vec![None; segments.len() + 1];

        calculate(
            segments,
            0,
            segments.len(),
            &mut real_time_predictions,
            false,
            false,
            TimingMethod::RealTime,
        );
        calculate(
            segments,
            0,
            segments.len(),
            &mut game_time_predictions,
            false,
            false,
            TimingMethod::GameTime,
        );

        for ((segment, &real_time), &game_time) in segments
            .iter_mut()
            .zip(real_time_predictions[1..].iter())
            .zip(game_time_predictions[1..].iter())
        {
            *segment.comparison_mut(NAME) = Time::new()
                .with_real_time(real_time)
                .with_game_time(game_time);
        }
    }
}
