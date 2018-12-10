//! Defines the Comparison Generator for the None comparison. The None
//! Comparison intentionally leaves all split times empty.

use super::ComparisonGenerator;
use crate::{Attempt, Segment, Time};

/// The Comparison Generator for the None comparison. The None Comparison
/// intentionally leaves all split times empty.
#[derive(Copy, Clone, Debug)]
pub struct None;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = NAME;
/// The name of this comparison.
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
