//! Provides different helper functions.

use crate::comparison::best_segments;
use crate::settings::SemanticColor;
use crate::{Run, Segment, TimeSpan, Timer, TimerPhase, TimingMethod};

/// Gets the last non-live delta in the run starting from `segment_index`.
///
/// - `run`: The current run.
/// - `segment_index`: The split number to start checking deltas from.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The timing method that you are using.
///
/// Returns the last non-live delta or None if there have been no deltas yet.
pub fn last_delta(
    run: &Run,
    segment_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    find_previous_non_empty_split_and_comparison_time(
        &run.segments()[..=segment_index],
        comparison,
        method,
    )
    .map(|(split_time, comparison_time)| split_time - comparison_time)
}

fn find_previous_non_empty_segment<F, T>(segments: &[Segment], check_segment: F) -> Option<T>
where
    F: FnMut(&Segment) -> Option<T>,
{
    segments.iter().rev().find_map(check_segment)
}

fn find_previous_non_empty_split_time(
    segments: &[Segment],
    method: TimingMethod,
) -> Option<TimeSpan> {
    find_previous_non_empty_segment(segments, |s| s.split_time()[method])
}

fn find_previous_non_empty_comparison_time(
    segments: &[Segment],
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    find_previous_non_empty_segment(segments, |s| s.comparison(comparison)[method])
}

fn find_previous_non_empty_split_and_comparison_time(
    segments: &[Segment],
    comparison: &str,
    method: TimingMethod,
) -> Option<(TimeSpan, TimeSpan)> {
    find_previous_non_empty_segment(segments, |s| {
        catch! {
            (s.split_time()[method]?, s.comparison(comparison)[method]?)
        }
    })
}

/// Calculates the comparison's segment time of the segment with the timing
/// method specified, combining segments if the segment before it is empty.
/// This is not calculating the current attempt's segment times.
///
/// # Panics
///
/// Panics if the provided `segment_index` is greater than or equal to
/// `run.len()`.
pub fn comparison_combined_segment_time(
    run: &Run,
    segment_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    if comparison == best_segments::NAME {
        return run.segment(segment_index).best_segment_time()[method];
    }

    let current_comparison_time = run.segment(segment_index).comparison(comparison)[method]?;

    let previous_comparison_time = find_previous_non_empty_comparison_time(
        &run.segments()[..segment_index],
        comparison,
        method,
    )
    .unwrap_or_default();

    Some(current_comparison_time - previous_comparison_time)
}

/// Calculates the comparison's segment time of the segment with the timing
/// method specified. This is not calculating the current attempt's segment
/// times.
///
/// # Panics
///
/// Panics if the provided `segment_index` is greater than or equal to
/// `run.len()`.
pub fn comparison_single_segment_time(
    run: &Run,
    segment_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    if comparison == best_segments::NAME {
        return run.segment(segment_index).best_segment_time()[method];
    }

    if segment_index == 0 {
        run.segment(segment_index).comparison(comparison)[method]
    } else {
        let current_comparison_time = run.segment(segment_index).comparison(comparison)[method]?;

        let previous_comparison_time =
            run.segment(segment_index - 1).comparison(comparison)[method]?;

        Some(current_comparison_time - previous_comparison_time)
    }
}

fn segment_delta(
    run: &Run,
    segment_index: usize,
    current_time: TimeSpan,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    let segment_index_comparison = run.segment(segment_index).comparison(comparison)[method]?;

    Some(
        find_previous_non_empty_split_and_comparison_time(
            &run.segments()[..segment_index],
            comparison,
            method,
        )
        .map(|(split_time, comparison_time)| {
            (current_time - segment_index_comparison) - (split_time - comparison_time)
        })
        .unwrap_or_else(|| current_time - segment_index_comparison),
    )
}

fn segment_time(
    run: &Run,
    segment_index: usize,
    current_time: TimeSpan,
    method: TimingMethod,
) -> TimeSpan {
    find_previous_non_empty_split_time(&run.segments()[..segment_index], method)
        .map(|split_time| current_time - split_time)
        .unwrap_or(current_time)
}

/// Gets the length of the last segment that leads up to a certain split.
///
/// - `timer`: The current timer.
/// - `segment_index`: The index of the split that represents the end of the
///   segment.
/// - `method`: The timing method that you are using.
///
/// Returns the length of the segment leading up to `segment_index`, returning
/// None if the split is not completed yet.
pub fn previous_segment_time(
    timer: &Timer,
    segment_index: usize,
    method: TimingMethod,
) -> Option<TimeSpan> {
    segment_time(
        timer.run(),
        segment_index,
        timer.run().segment(segment_index).split_time()[method]?,
        method,
    )
    .into()
}

/// Gets the length of the last segment that leads up to a certain split, using
/// the live segment time if the split is not completed yet.
///
/// - `timer`: The current timer.
/// - `segment_index`: The index of the split that represents the end of the
///   segment.
/// - `method`: The timing method that you are using.
///
/// Returns the length of the segment leading up to `segment_index`, returning
/// the live segment time if the split is not completed yet.
pub fn live_segment_time(
    timer: &Timer,
    segment_index: usize,
    method: TimingMethod,
) -> Option<TimeSpan> {
    segment_time(
        timer.run(),
        segment_index,
        timer.current_time()[method]?,
        method,
    )
    .into()
}

/// Gets the amount of time lost or gained on a certain split.
///
/// - `timer`: The current timer.
/// - `segment_index`: The index of the split for which the delta is calculated.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The timing method that you are using.
///
/// Returns the segment delta for a certain split, returning None if the split
/// is not completed yet.
pub fn previous_segment_delta(
    timer: &Timer,
    segment_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    segment_delta(
        timer.run(),
        segment_index,
        timer.run().segment(segment_index).split_time()[method]?,
        comparison,
        method,
    )
}

/// Gets the amount of time lost or gained on a certain split, using the live
/// segment delta if the split is not completed yet.
///
/// - `timer`: The current timer.
/// - `segment_index`: The index of the split for which the delta is calculated.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The timing method that you are using.
///
/// Returns the segment delta for a certain split, returning the live segment
/// delta if the split is not completed yet.
pub fn live_segment_delta(
    timer: &Timer,
    segment_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    segment_delta(
        timer.run(),
        segment_index,
        timer.current_time()[method]?,
        comparison,
        method,
    )
}

/// Checks whether the live segment should now be shown.
///
/// - `timer`: The current timer.
/// - `split_delta`: Specifies whether to return a split delta
///    rather than a segment delta and to start showing the live
///    segment once you are behind.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The timing method that you are using.
///
/// Returns the current live delta.
pub fn check_live_delta(
    timer: &Timer,
    split_delta: bool,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    if timer.current_phase() == TimerPhase::Running || timer.current_phase() == TimerPhase::Paused {
        let current_split = timer
            .current_split()
            .unwrap()
            .comparison_timing_method(comparison, method);
        let current_time = timer.current_time()[method];
        let segment_index = timer.current_split_index().unwrap();
        let current_segment = live_segment_time(timer, segment_index, method);
        let best_segment = timer.run().segment(segment_index).best_segment_time()[method];
        let best_segment_delta =
            live_segment_delta(timer, segment_index, best_segments::NAME, method);
        let comparison_delta = live_segment_delta(timer, segment_index, comparison, method);

        if split_delta && current_time > current_split
            || catch! { current_segment? > best_segment? }.unwrap_or(false)
                && best_segment_delta.map_or(false, |d| d > TimeSpan::zero())
            || comparison_delta.map_or(false, |d| d > TimeSpan::zero())
        {
            if split_delta {
                return catch! { current_time? - current_split? };
            } else {
                return comparison_delta;
            }
        }
    }
    None
}

/// Chooses a split color from the Layout Settings based on the current run.
///
/// - `timer`: The current timer.
/// - `time_difference`: The delta that you want to find a color for.
/// - `segment_index`: The split number that is associated with this delta.
/// - `show_segment_deltas`: Can show ahead gaining and behind losing colors if
///    true.
/// - `show_best_segments`: Can show the best segment color if true.
/// - `comparison`: The comparison that you are comparing this delta to.
/// - `method`: The timing method of this delta.
///
/// Returns the chosen color.
pub fn split_color(
    timer: &Timer,
    time_difference: Option<TimeSpan>,
    segment_index: usize,
    show_segment_deltas: bool,
    show_best_segments: bool,
    comparison: &str,
    method: TimingMethod,
) -> SemanticColor {
    if show_best_segments && check_best_segment(timer, segment_index, method) {
        SemanticColor::BestSegment
    } else if let Some(time_difference) = time_difference {
        let last_delta = segment_index
            .checked_sub(1)
            .and_then(|n| last_delta(timer.run(), n, comparison, method));
        if time_difference < TimeSpan::zero() {
            if show_segment_deltas && last_delta.map_or(false, |d| time_difference > d) {
                SemanticColor::AheadLosingTime
            } else {
                SemanticColor::AheadGainingTime
            }
        } else if show_segment_deltas && last_delta.map_or(false, |d| time_difference < d) {
            SemanticColor::BehindGainingTime
        } else {
            SemanticColor::BehindLosingTime
        }
    } else {
        SemanticColor::Default
    }
}

/// Calculates whether or not the Split Times for the indicated split qualify as
/// a Best Segment.
///
/// - `timer`: The current timer.
/// - `segment_index`: The split to check.
/// - `method`: The timing method to use.
///
/// Returns whether or not the indicated split is a Best Segment.
pub fn check_best_segment(timer: &Timer, segment_index: usize, method: TimingMethod) -> bool {
    if timer.run().segment(segment_index).split_time()[method].is_none() {
        return false;
    }

    let delta = previous_segment_delta(timer, segment_index, best_segments::NAME, method);
    let current_segment = previous_segment_time(timer, segment_index, method);
    let best_segment = timer.run().segment(segment_index).best_segment_time()[method];
    best_segment.map_or(true, |b| {
        current_segment.map_or(false, |c| c < b) || delta.map_or(false, |d| d < TimeSpan::zero())
    })
}
