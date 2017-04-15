use super::ComparisonGenerator;
use {Attempt, Segment, Time};
use clone_on_write::Cow;

#[derive(Copy, Clone, Debug)]
pub struct None;

pub const NAME: &'static str = "None";

impl ComparisonGenerator for None {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Cow<Segment>], _: &[Attempt]) {
        for segment in segments {
            *segment.comparison_mut(NAME) = Time::default();
        }
    }
}
