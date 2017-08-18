use {TimeSpan, Timer, TimerPhase};
use analysis;

pub fn calculate(timer: &Timer, comparison: &str) -> Option<TimeSpan> {
    let timing_method = timer.current_timing_method();
    let last_segment = timer.run().segments.last().unwrap();

    match timer.current_phase() {
        TimerPhase::Running | TimerPhase::Paused => {
            let mut delta = analysis::last_delta(
                timer.run(),
                timer.current_split_index() as usize,
                comparison,
                timing_method,
            ).unwrap_or_default();

            let live_delta = TimeSpan::option_sub(
                timer.current_time()[timing_method],
                timer.current_split().unwrap().comparison(comparison)[timing_method],
            );

            if let Some(live_delta) = live_delta {
                if live_delta > delta {
                    delta = live_delta;
                }
            }

            last_segment.comparison(comparison)[timing_method].map(|c| delta + c)
        }
        TimerPhase::Ended => last_segment.split_time[timing_method],
        TimerPhase::NotRunning => last_segment.comparison(comparison)[timing_method],
    }
}
