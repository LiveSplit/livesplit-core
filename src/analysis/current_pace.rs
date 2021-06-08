//! Calculates the current pace of the active attempt based on the comparison
//! provided. If there's no active attempt, the final time of the comparison is
//! returned instead.

use crate::{analysis, timing::Snapshot, TimeSpan, TimerPhase};

/// Calculates the current pace of the active attempt based on the comparison
/// provided. If there's no active attempt, the final time of the comparison is
/// returned instead.
pub fn calculate(timer: &Snapshot<'_>, comparison: &str) -> (Option<TimeSpan>, bool) {
    let timing_method = timer.current_timing_method();
    let last_segment = timer.run().segments().last().unwrap();

    match timer.current_phase() {
        TimerPhase::Running | TimerPhase::Paused => {
            let mut delta = analysis::last_delta(
                timer.run(),
                timer.current_split_index().unwrap(),
                comparison,
                timing_method,
            )
            .unwrap_or_default();

            let mut is_live = false;

            catch! {
                let live_delta = timer.current_time()[timing_method]?
                    - timer.current_split().unwrap().comparison(comparison)[timing_method]?;

                if live_delta > delta {
                    delta = live_delta;
                    is_live = true;
                }
            };

            let value = catch! {
                last_segment.comparison(comparison)[timing_method]? + delta
            };

            (
                value,
                is_live && timer.current_phase().is_running() && value.is_some(),
            )
        }
        TimerPhase::Ended => (last_segment.split_time()[timing_method], false),
        TimerPhase::NotRunning => (last_segment.comparison(comparison)[timing_method], false),
    }
}
