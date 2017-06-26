use super::ComparisonGenerator;
use {Attempt, Segment, Time};

#[derive(Copy, Clone, Debug)]
pub struct None;

pub const SHORT_NAME: &str = NAME;
pub const NAME: &str = "None";

impl ComparisonGenerator for None {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        for segment in segments {
            *segment.comparison_mut(NAME) = Time::default();
        }
    }
}
