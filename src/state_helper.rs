use {Color, Run, Timer, TimingMethod, TimeSpan, TimerPhase};
use run::PERSONAL_BEST_COMPARISON_NAME;
use comparison::best_segments;

/// Gets the last non-live delta in the run starting from `split_number`.
///
/// `run`: The current run.
/// `split_number`: The split number to start checking deltas from.
/// `comparison`: The comparison that you are comparing with.
/// `method`: The timing method that you are using.
///
/// Returns the last non-live delta or None if there have been no deltas yet.
pub fn last_delta(run: &Run,
                  split_number: usize,
                  comparison: &str,
                  method: TimingMethod)
                  -> Option<TimeSpan> {
    for segment in run.segments()[..split_number + 1].iter().rev() {
        let comparison = segment.comparison_timing_method(comparison, method);
        let split_time = segment.split_time()[method];
        if let (Some(comparison), Some(split_time)) = (comparison, split_time) {
            return Some(split_time - comparison);
        }
    }
    None
}

fn segment_time_or_segment_delta(timer: &Timer,
                                 split_number: usize,
                                 use_current_time: bool,
                                 segment_time: bool,
                                 comparison: &str,
                                 method: TimingMethod)
                                 -> Option<TimeSpan> {
    let current_time = if use_current_time {
        timer.current_time()[method]
    } else {
        timer.run().segment(split_number).split_time()[method]
    };

    if let Some(current_time) = current_time {
        let split_number_comparison =
            timer.run().segment(split_number).comparison_timing_method(comparison, method);

        for segment in timer.run().segments()[..split_number].iter().rev() {
            if let Some(split_time) = segment.split_time()[method] {
                if segment_time {
                    return Some(current_time - split_time);
                } else if let Some(comparison) =
                    segment.comparison_timing_method(comparison, method) {
                    return split_number_comparison.map(|s| (current_time - s) - (split_time - comparison));
                }
            }
        }

        if segment_time {
            Some(current_time)
        } else {
            split_number_comparison.map(|s| current_time - s)
        }
    } else {
        None
    }
}

/// Gets the length of the last segment that leads up to a certain split.
///
/// `timer`: The current timer.
/// `split_number`: The index of the split that represents the end of the segment.
/// `method`: The timing method that you are using.
///
/// Returns the length of the segment leading up to `split_number`, returning None if the split is not completed yet.
pub fn previous_segment_time(timer: &Timer,
                             split_number: usize,
                             method: TimingMethod)
                             -> Option<TimeSpan> {
    segment_time_or_segment_delta(timer,
                                  split_number,
                                  false,
                                  true,
                                  PERSONAL_BEST_COMPARISON_NAME,
                                  method)
}

/// Gets the length of the last segment that leads up to a certain split, using the live segment time if the split is not completed yet.
///
/// `timer`: The current timer.
/// `split_number`: The index of the split that represents the end of the segment.
/// `method`: The timing method that you are using.
///
/// Returns the length of the segment leading up to `split_number`, returning the live segment time if the split is not completed yet.
pub fn live_segment_time(timer: &Timer,
                         split_number: usize,
                         method: TimingMethod)
                         -> Option<TimeSpan> {
    segment_time_or_segment_delta(timer,
                                  split_number,
                                  true,
                                  true,
                                  PERSONAL_BEST_COMPARISON_NAME,
                                  method)
}

/// Gets the amount of time lost or gained on a certain split.
///
/// `timer`: The current timer.
/// `split_number`: The index of the split for which the delta is calculated.
/// `comparison`: The comparison that you are comparing with.
/// `method`: The timing method that you are using.
///
/// Returns the segment delta for a certain split, returning None if the split is not completed yet.
pub fn previous_segment_delta(timer: &Timer,
                              split_number: usize,
                              comparison: &str,
                              method: TimingMethod)
                              -> Option<TimeSpan> {
    segment_time_or_segment_delta(timer, split_number, false, false, comparison, method)
}

/// Gets the amount of time lost or gained on a certain split, using the live segment delta if the split is not completed yet.
///
/// `timer`: The current timer.
/// `split_number`: The index of the split for which the delta is calculated.
/// `comparison`: The comparison that you are comparing with.
/// `method`: The timing method that you are using.
///
/// Returns the segment delta for a certain split, returning the live segment delta if the split is not completed yet.
pub fn live_segment_delta(timer: &Timer,
                          split_number: usize,
                          comparison: &str,
                          method: TimingMethod)
                          -> Option<TimeSpan> {
    segment_time_or_segment_delta(timer, split_number, true, false, comparison, method)
}

/// Checks whether the live segment should now be shown.
///
/// `timer`: The current timer.
/// `show_when_behind`: Specifies whether or not to start showing the live segment once you are behind.
/// `comparison`: The comparison that you are comparing with.
/// `method`: The timing method that you are using.
///
/// Returns the current live delta.
pub fn check_live_delta(timer: &Timer,
                        show_when_behind: bool,
                        comparison: &str,
                        method: TimingMethod)
                        -> Option<TimeSpan> {
    if timer.current_phase() == TimerPhase::Running || timer.current_phase() == TimerPhase::Paused {
        let use_best_segment = true; // TODO Make this a parameter
        let current_split =
            timer.current_split().unwrap().comparison_timing_method(comparison, method);
        let current_time = timer.current_time()[method];
        let split_index = timer.current_split_index() as usize;
        let current_segment = live_segment_time(timer, split_index, method);
        let best_segment = timer.run().segment(split_index).best_segment_time()[method];
        let best_segment_delta =
            live_segment_delta(timer, split_index, best_segments::NAME, method);
        let comparison_delta = live_segment_delta(timer, split_index, comparison, method);

        if show_when_behind && current_time > current_split ||
           use_best_segment &&
           TimeSpan::option_op(current_segment, best_segment, |c, b| c > b).unwrap_or(false) &&
           best_segment_delta.map_or(false, |d| d > TimeSpan::zero()) ||
           comparison_delta.map_or(false, |d| d > TimeSpan::zero()) {
            return TimeSpan::option_op(current_time, current_split, |t, s| t - s);
        }
    }
    None
}

/// Chooses a split color from the Layout Settings based on the current run.
///
/// `timer`: The current timer.
/// `time_difference`: The delta that you want to find a color for.
/// `split_number`: The split number that is associated with this delta.
/// `show_segment_deltas`: Can show ahead gaining and behind losing colors if true.
/// `show_best_segments`: Can show the best segment color if true.
/// `comparison`: The comparison that you are comparing this delta to.
/// `method`: The timing method of this delta.
///
/// Returns the chosen color.
pub fn split_color(timer: &Timer,
                   time_difference: Option<TimeSpan>,
                   split_number: usize,
                   show_segment_deltas: bool,
                   show_best_segments: bool,
                   comparison: &str,
                   method: TimingMethod)
                   -> Color {
    let use_best_segment = true; // TODO Make this a parameter

    if show_best_segments && use_best_segment && check_best_segment(timer, split_number, method) {
        Color::BestSegment
    } else if let Some(time_difference) = time_difference {
        let last_delta = split_number.checked_sub(1)
            .and_then(|n| last_delta(timer.run(), n, comparison, method));
        if time_difference < TimeSpan::zero() {
            if show_segment_deltas && last_delta.map_or(false, |d| time_difference > d) {
                Color::AheadLosingTime
            } else {
                Color::AheadGainingTime
            }
        } else if show_segment_deltas && last_delta.map_or(false, |d| time_difference < d) {
            Color::BehindGainingTime
        } else {
            Color::BehindLosingTime
        }
    } else {
        Color::Default
    }
}

/// Calculates whether or not the Split Times for the indicated split qualify as a Best Segment.
///
/// `timer`: The current timer.
/// `split_number`: The split to check.
/// `method`: The timing method to use.
///
/// Returns whether or not the indicated split is a Best Segment.
pub fn check_best_segment(timer: &Timer, split_number: usize, method: TimingMethod) -> bool {
    if timer.run().segment(split_number).split_time()[method].is_none() {
        return false;
    }

    let delta = previous_segment_delta(timer, split_number, best_segments::NAME, method);
    let current_segment = previous_segment_time(timer, split_number, method);
    let best_segment = timer.run().segment(split_number).best_segment_time()[method];
    best_segment.map_or(true, |b| {
        current_segment.map_or(false, |c| c < b) || delta.map_or(false, |d| d < TimeSpan::zero())
    })
}
