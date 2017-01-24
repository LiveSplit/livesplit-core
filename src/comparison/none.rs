use super::ComparisonGenerator;
use {Segment, Time};

#[derive(Copy, Clone, Debug)]
pub struct None;

pub const NAME: &'static str = "None";

impl ComparisonGenerator for None {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment]) {
        for segment in segments {
            *segment.comparison_mut(NAME) = Time::default();
        }
    }
}
