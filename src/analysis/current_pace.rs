use {Timer, TimeSpan, TimerPhase};
use analysis;

pub fn calculate(timer: &Timer, comparison: Option<&str>) -> Option<TimeSpan> {
    let comparison = comparison.unwrap_or_else(|| timer.current_comparison());

    let timing_method = timer.current_timing_method();
    let last_segment = timer.run().segments().iter().last().unwrap();

    match timer.current_phase() {
        TimerPhase::Running | TimerPhase::Paused => {
            let mut delta = analysis::last_delta(timer.run(),
                                                 timer.current_split_index() as usize,
                                                 comparison,
                                                 timing_method);

            let live_delta =
                TimeSpan::option_op(timer.current_time()[timing_method],
                                    timer.current_split().unwrap().comparison(comparison)
                                        [timing_method],
                                    |a, b| a - b);

            if TimeSpan::option_op(live_delta, delta, |a, b| a > b).unwrap_or(false) {
                delta = live_delta;
            }

            TimeSpan::option_op(delta,
                                last_segment.comparison(comparison)[timing_method],
                                |a, b| a + b)
        }
        TimerPhase::Ended => last_segment.split_time()[timing_method],
        TimerPhase::NotRunning => last_segment.comparison(comparison)[timing_method],
    }
}
