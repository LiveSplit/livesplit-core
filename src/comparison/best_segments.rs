use super::ComparisonGenerator;
use {Attempt, Segment, Time, TimingMethod};
use sum_of_segments::best::calculate;
use clone_on_write::Cow;

#[derive(Copy, Clone, Debug)]
pub struct BestSegments;

pub const NAME: &'static str = "Best Segments";

impl ComparisonGenerator for BestSegments {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Cow<Segment>], _: &[Attempt]) {
        let mut real_time_predictions = vec![None; segments.len() + 1];
        let mut game_time_predictions = vec![None; segments.len() + 1];

        calculate(segments,
                  0,
                  segments.len(),
                  &mut real_time_predictions,
                  false,
                  false,
                  TimingMethod::RealTime);
        calculate(segments,
                  0,
                  segments.len(),
                  &mut game_time_predictions,
                  false,
                  false,
                  TimingMethod::GameTime);

        for ((segment, &real_time), &game_time) in
            segments.iter_mut()
                .zip(real_time_predictions[1..].iter())
                .zip(game_time_predictions[1..].iter()) {
            *segment.comparison_mut(NAME) = Time::new()
                .with_real_time(real_time)
                .with_game_time(game_time);
        }
    }
}
