//! The comparison module provides all the different automatically generated
//! comparisons, like the Best Segments and the Average Segments. Additionally,
//! functions for dealing with comparisons, like shortening a comparison, are
//! provided.

#[cfg(test)]
mod tests;

pub mod average_segments;
pub mod balanced_pb;
pub mod best_segments;
pub mod best_split_times;
pub mod goal;
pub mod latest_run;
pub mod median_segments;
pub mod none;
pub mod worst_segments;

pub use self::average_segments::AverageSegments;
pub use self::balanced_pb::BalancedPB;
pub use self::best_segments::BestSegments;
pub use self::best_split_times::BestSplitTimes;
pub use self::latest_run::LatestRun;
pub use self::median_segments::MedianSegments;
pub use self::none::None;
pub use self::worst_segments::WorstSegments;

use crate::platform::prelude::*;
use crate::{Attempt, Segment, Timer};
use core::fmt::Debug;

/// Defines the Personal Best comparison. This module mostly just serves for
/// providing the names of the comparison, as the Personal Best is not a
/// Comparison Generator.
pub mod personal_best {
    /// The short name of this comparison. Suitable for situations where not a lot
    /// of space for text is available.
    pub const SHORT_NAME: &str = "PB";
    /// The name of this comparison.
    pub const NAME: &str = "Personal Best";
}

/// Defines the World Record comparison. This module mostly just serves for
/// providing the names of the comparison, as the World Record is not a
/// Comparison Generator.
pub mod world_record {
    /// The short name of this comparison. Suitable for situations where not a lot
    /// of space for text is available.
    pub const SHORT_NAME: &str = "WR";
    /// The name of this comparison.
    pub const NAME: &str = "World Record";
}

/// A Comparison Generator automatically generates a comparison based on what
/// kind of generator it is. Comparison Generators stored in a Run automatically
/// get called between all attempts to refresh the comparison's information.
pub trait ComparisonGenerator: Debug + Sync + Send + ComparisonGeneratorClone {
    /// The name of the comparison.
    fn name(&self) -> &str;
    /// Generate the comparison. The comparison generator is expected to modify
    /// the comparison's times for each segment. The Attempt History is
    /// provided, in case the comparison generator requires information from the
    /// previous attempts.
    fn generate(&mut self, segments: &mut [Segment], attempts: &[Attempt]);
}

/// Provides the ability to clone a Comparison Generator, even when it is stored
/// as a Trait Object.
pub trait ComparisonGeneratorClone {
    /// Clones the Comparison Generator as a Trait Object.
    fn clone_box(&self) -> Box<dyn ComparisonGenerator>;
}

impl<T> ComparisonGeneratorClone for T
where
    T: 'static + ComparisonGenerator + Clone,
{
    fn clone_box(&self) -> Box<dyn ComparisonGenerator> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ComparisonGenerator> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Creates a list of all the Comparison Generators that are active by default.
/// Which comparison generators are in this list may change in future versions.
pub fn default_generators() -> Vec<Box<dyn ComparisonGenerator>> {
    vec![
        Box::new(BestSegments),
        Box::new(BestSplitTimes),
        Box::new(AverageSegments),
        Box::new(MedianSegments),
        Box::new(WorstSegments),
        Box::new(BalancedPB),
        Box::new(LatestRun),
        Box::new(None),
    ]
}

/// Shortens a comparison name. If the name of the comparison matches one of the
/// comparison generators, the short name of that comparison generator is
/// returned. Otherwise the comparison name is returned without being shortened.
/// Additional shortening logic for other comparison names may happen in the
/// future.
pub fn shorten(comparison: &str) -> &str {
    match comparison {
        personal_best::NAME => personal_best::SHORT_NAME,
        world_record::NAME => world_record::SHORT_NAME,
        average_segments::NAME => average_segments::SHORT_NAME,
        median_segments::NAME => median_segments::SHORT_NAME,
        balanced_pb::NAME => balanced_pb::SHORT_NAME,
        best_segments::NAME => best_segments::SHORT_NAME,
        best_split_times::NAME => best_split_times::SHORT_NAME,
        latest_run::NAME => latest_run::SHORT_NAME,
        none::NAME => none::SHORT_NAME,
        worst_segments::NAME => worst_segments::SHORT_NAME,
        c => c,
    }
}

/// Helper function for accessing either the given comparison or a Timer's
/// current comparison if the given comparison is `None`.
pub fn or_current<'a>(comparison: Option<&'a str>, timer: &'a Timer) -> &'a str {
    comparison.unwrap_or_else(|| timer.current_comparison())
}

/// Tries to resolve the given comparison based on a Timer object. If either
/// `None` is given or the comparison doesn't exist, `None` is returned.
/// Otherwise the comparison name stored in the Timer is returned by reference.
pub fn resolve<'a>(comparison: &Option<String>, timer: &'a Timer) -> Option<&'a str> {
    let comparison = comparison.as_ref()?;
    timer.run().comparisons().find(|&rc| comparison == rc)
}
