//! Provides functionality to calculate the total playtime for either a Run or a
//! Timer. For a Run, all the durations stored in the Attempt History are summed
//! together. For a Timer, the current attempt's duration is also factored in.

use crate::{Run, TimeSpan, Timer, TimingMethod};

/// Allows calculating the total playtime.
pub trait TotalPlaytime {
    /// Calculates the total playtime.
    fn total_playtime(&self) -> TimeSpan;
}

impl TotalPlaytime for Run {
    fn total_playtime(&self) -> TimeSpan {
        let mut total_playtime = TimeSpan::zero();

        for attempt in self.attempt_history() {
            if let Some(duration) = attempt.duration() {
                // Either >= 1.6.0 or a finished run
                total_playtime += duration;
                if let Some(pause_time) = attempt.pause_time() {
                    total_playtime -= pause_time;
                }
            } else {
                // Must be < 1.6.0 and a reset
                // Calculate the sum of the segments for that run
                for segment in self.segments() {
                    if let Some(segment_time) = segment
                        .segment_history()
                        .get(attempt.index())
                        .and_then(|s| s[TimingMethod::RealTime])
                    {
                        total_playtime += segment_time;
                    }
                }
            }
        }

        total_playtime
    }
}

impl TotalPlaytime for Timer {
    fn total_playtime(&self) -> TimeSpan {
        let timer_play_time =
            self.current_attempt_duration() - self.get_pause_time().unwrap_or_default();
        let run_play_time = self.run().total_playtime();

        timer_play_time + run_play_time
    }
}

impl<'a, T: 'a + TotalPlaytime> TotalPlaytime for &'a T {
    fn total_playtime(&self) -> TimeSpan {
        (*self).total_playtime()
    }
}

/// Calculates the total playtime. The source can be a `Run`, `Timer` or any
/// other type that implements the `TotalPlaytime` trait.
pub fn calculate<T: TotalPlaytime>(source: T) -> TimeSpan {
    source.total_playtime()
}
