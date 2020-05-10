//! Defines the Comparison Generator for calculating a comparison which has the
//! same final time as the runner's Personal Best. Unlike the Personal Best
//! however, all the other split times are automatically balanced by the
//! runner's history in order to balance out the mistakes present in the
//! Personal Best throughout the comparison. Running against an unbalanced
//! Personal Best can cause frustrations. A Personal Best with a mediocre early
//! game and a really good end game has a high chance of the runner losing a lot
//! of time compared to the Personal Best towards the end of a run. This may
//! discourage the runner, which may lead them to reset the attempt. That's the
//! perfect situation to compare against the Balanced Personal Best comparison
//! instead, as all of the mistakes of the early game in such a situation would
//! be smoothed out throughout the whole comparison.
//!
//! The algorithm is sampling the split times on the skill curve where the
//! Personal Best is located.

use super::{goal, ComparisonGenerator};
use crate::{analysis::SkillCurve, Attempt, Segment, TimingMethod};

/// The Comparison Generator for calculating a comparison which has the same
/// final time as the runner's Personal Best. Unlike the Personal Best however,
/// all the other split times are automatically balanced by the runner's history
/// in order to balance out the mistakes present in the Personal Best throughout
/// the comparison. Running against an unbalanced Personal Best can cause
/// frustrations. A Personal Best with a mediocre early game and a really good
/// end game has a high chance of the runner losing a lot of time compared to
/// the Personal Best towards the end of a run. This may discourage the runner,
/// which may lead them to reset the attempt. That's the perfect situation to
/// compare against the Balanced Personal Best comparison instead, as all of the
/// mistakes of the early game in such a situation would be smoothed out
/// throughout the whole comparison.
///
/// The algorithm is sampling the split times on the skill curve where the
/// Personal Best is located.
#[derive(Copy, Clone, Debug)]
pub struct BalancedPB;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Balanced";
/// The name of this comparison.
pub const NAME: &str = "Balanced PB";

impl ComparisonGenerator for BalancedPB {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        let mut skill_curve = SkillCurve::new();

        goal::generate_for_timing_method_with_buf(
            segments,
            TimingMethod::RealTime,
            None,
            NAME,
            &mut skill_curve,
        );
        goal::generate_for_timing_method_with_buf(
            segments,
            TimingMethod::GameTime,
            None,
            NAME,
            &mut skill_curve,
        );
    }
}
