//! The analysis module provides a variety of functions for calculating
//! information about runs.

use super::time_span::{NullableOwnedTimeSpan, OwnedTimeSpan};
use livesplit_core::{
    Run, Timer, TimingMethod,
    analysis::{sum_of_segments::calculate_best, total_playtime::calculate},
};

/// Calculates the Sum of Best Segments for the timing method provided. This is
/// the fastest time possible to complete a run of a category, based on
/// information collected from all the previous attempts. This often matches up
/// with the sum of the best segment times of all the segments, but that may not
/// always be the case, as skipped segments may introduce combined segments that
/// may be faster than the actual sum of their best segment times. The name is
/// therefore a bit misleading, but sticks around for historical reasons. You
/// can choose to do a simple calculation instead, which excludes the Segment
/// History from the calculation process. If there's an active attempt, you can
/// choose to take it into account as well. Can return <NULL>.
#[unsafe(no_mangle)]
pub extern "C" fn Analysis_calculate_sum_of_best(
    run: &Run,
    simple_calculation: bool,
    use_current_run: bool,
    method: TimingMethod,
) -> NullableOwnedTimeSpan {
    calculate_best(run.segments(), simple_calculation, use_current_run, method).map(Box::new)
}

/// Calculates the total playtime of the passed Run.
#[unsafe(no_mangle)]
pub extern "C" fn Analysis_calculate_total_playtime_for_run(run: &Run) -> OwnedTimeSpan {
    Box::new(calculate(run))
}

/// Calculates the total playtime of the passed Timer.
#[unsafe(no_mangle)]
pub extern "C" fn Analysis_calculate_total_playtime_for_timer(timer: &Timer) -> OwnedTimeSpan {
    Box::new(calculate(timer))
}
