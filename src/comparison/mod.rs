pub mod average_segments;
pub mod best_segments;
pub mod best_split_times;
pub mod none;
pub mod worst_segments;
pub mod most_recent_run;

pub use self::average_segments::AverageSegments;
pub use self::best_segments::BestSegments;
pub use self::best_split_times::BestSplitTimes;
pub use self::none::None;
pub use self::worst_segments::WorstSegments;
pub use self::most_recent_run::MostRecentRun;

use std::fmt::Debug;
use {Attempt, Segment};

pub trait ComparisonGenerator: Debug + Sync + Send + ComparisonGeneratorClone {
    fn name(&self) -> &str;
    fn generate(&mut self, segments: &mut [Segment], attempts: &[Attempt]);
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
    vec![Box::new(BestSegments),
         Box::new(BestSplitTimes),
         Box::new(AverageSegments),
         Box::new(WorstSegments),
         Box::new(MostRecentRun),
         Box::new(None)]
}
