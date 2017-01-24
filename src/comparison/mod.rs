pub mod best_segments;
pub mod none;
pub mod worst_segments;

pub use self::best_segments::BestSegments;
pub use self::none::None;
pub use self::worst_segments::WorstSegments;

use std::fmt::Debug;
use Segment;

pub trait ComparisonGenerator: Debug + ComparisonGeneratorClone {
    fn name(&self) -> &str;
    fn generate(&mut self, segments: &mut [Segment]);
}

pub trait ComparisonGeneratorClone {
    fn clone_box(&self) -> Box<ComparisonGenerator>;
}

impl<T> ComparisonGeneratorClone for T
    where T: 'static + ComparisonGenerator + Clone
{
    fn clone_box(&self) -> Box<ComparisonGenerator> {
        Box::new(self.clone())
    }
}

impl Clone for Box<ComparisonGenerator> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub fn default_generators() -> Vec<Box<ComparisonGenerator>> {
    vec![Box::new(BestSegments), Box::new(WorstSegments), Box::new(None)]
}
