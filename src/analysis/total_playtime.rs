use {Timer, TimeSpan, TimingMethod};

pub fn calculate(timer: &Timer) -> TimeSpan {
    let mut total_playtime = TimeSpan::zero();

    for attempt in timer.run().attempt_history() {
        if let Some(duration) = attempt.duration() {
            // Either >= 1.6.0 or a finished run
            total_playtime += duration;
            if let Some(pause_time) = attempt.pause_time() {
                total_playtime -= pause_time;
            }
        } else {
            // Must be < 1.6.0 and a reset
            // Calculate the sum of the segments for that run
            for segment in timer.run().segments() {
                if let Some(segment_time) =
                    segment
                        .segment_history()
                        .get(attempt.index())
                        .and_then(|s| s[TimingMethod::RealTime]) {
                    total_playtime += segment_time;
                }
            }
        }
    }

    total_playtime += timer.current_attempt_duration() - timer.get_pause_time().unwrap_or_default();

    total_playtime
}
