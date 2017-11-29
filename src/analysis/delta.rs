use {TimeSpan, Timer, TimerPhase};
use analysis;

pub fn calculate(timer: &Timer, comparison: &str) -> (Option<TimeSpan>, bool) {
    let timing_method = timer.current_timing_method();
    let last_segment = timer.run().segments().last().unwrap();

    let mut use_live_delta = false;

    let time = match timer.current_phase() {
        TimerPhase::Running | TimerPhase::Paused => {
            let mut delta = analysis::last_delta(
                timer.run(),
                timer.current_split_index().unwrap(),
                comparison,
                timing_method,
            );

            catch! {
                let live_delta = timer.current_time()[timing_method]?
                    - timer.current_split().unwrap().comparison(comparison)[timing_method]?;

                if live_delta > delta.unwrap_or_default() {
                    delta = Some(live_delta);
                    use_live_delta = true;
                }
            };

            delta
        }
        TimerPhase::Ended => catch! {
            last_segment.split_time()[timing_method]?
                - last_segment.comparison(comparison)[timing_method]?
        },
        TimerPhase::NotRunning => None,
    };

    (time, use_live_delta)
}
