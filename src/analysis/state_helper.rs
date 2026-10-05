//! Provides different helper functions.

use crate::{
    Run, Segment, TimeSpan, Timer, TimerPhase, TimingMethod, analysis::sum_of_segments::Prediction,
    comparison::best_segments, settings::SemanticColor, timing::Snapshot,
};
use smallvec::{SmallVec, smallvec};

/// Gets the last non-live delta in the [`Run`] starting from `segment_index`.
///
/// - `run`: The current [`Run`].
/// - `segment_index`: The split number to start checking deltas from.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The [`TimingMethod`] that you are using.
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

/// Calculates the comparison's segment time over an inclusive range of
/// segments, combining it with the segment before the range if needed. This is
/// not calculating the current attempt's segment times.
///
/// # Panics
///
/// Panics if the provided `end_index` is greater than or equal to `run.len()`.
pub fn comparison_segment_time_for_range(
    run: &Run,
    start_index: usize,
    end_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    if start_index == end_index {
        return comparison_combined_segment_time(run, end_index, comparison, method);
    }

    if comparison == best_segments::NAME {
        let mut predictions: SmallVec<[Option<Prediction>; 64]> = smallvec![None; run.len() + 1];
        return super::sum_of_segments::best::calculate_segment_range(
            run.segments(),
            start_index,
            end_index + 1,
            &mut predictions,
            false,
            false,
            method,
        );
    }

    let current_comparison_time = run.segment(end_index).comparison(comparison)[method]?;
    let previous_comparison_time =
        find_previous_non_empty_comparison_time(&run.segments()[..start_index], comparison, method)
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
    start_index: usize,
    end_index: usize,
    current_time: TimeSpan,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    if start_index != end_index && comparison == best_segments::NAME {
        return Some(
            segment_time(run, start_index, current_time, method)
                - comparison_segment_time_for_range(
                    run,
                    start_index,
                    end_index,
                    comparison,
                    method,
                )?,
        );
    }

    let segment_index_comparison = run.segment(end_index).comparison(comparison)[method]?;

    Some(
        find_previous_non_empty_split_and_comparison_time(
            &run.segments()[..start_index],
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
    start_index: usize,
    current_time: TimeSpan,
    method: TimingMethod,
) -> TimeSpan {
    find_previous_non_empty_split_time(&run.segments()[..start_index], method)
        .map(|split_time| current_time - split_time)
        .unwrap_or(current_time)
}

/// Gets the length of the last segment that leads up to a certain split.
///
/// - `timer`: The current [`Timer`].
/// - `segment_index`: The index of the split that represents the end of the
///   segment.
/// - `method`: The [`TimingMethod`] that you are using.
///
/// Returns the length of the segment leading up to `segment_index`, returning
/// None if the split is not completed yet.
pub fn previous_segment_time(
    timer: &Timer,
    segment_index: usize,
    method: TimingMethod,
) -> Option<TimeSpan> {
    previous_segment_time_for_range(timer, segment_index, segment_index, method)
}

/// Gets the length of the last segment range that leads up to a certain split.
///
/// - `timer`: The current [`Timer`].
/// - `start_index`: The index of the first split in the range.
/// - `end_index`: The index of the split that represents the end of the range.
/// - `method`: The [`TimingMethod`] that you are using.
///
/// Returns the length of the segment range leading up to `end_index`, returning
/// None if the split is not completed yet.
pub fn previous_segment_time_for_range(
    timer: &Timer,
    start_index: usize,
    end_index: usize,
    method: TimingMethod,
) -> Option<TimeSpan> {
    segment_time(
        timer.run(),
        start_index,
        timer.run().segment(end_index).split_time()[method]?,
        method,
    )
    .into()
}

/// Gets the length of the last segment that leads up to a certain split, using
/// the live segment time if the split is not completed yet.
///
/// - `timer`: The current [`Timer`].
/// - `segment_index`: The index of the split that represents the end of the
///   segment.
/// - `method`: The [`TimingMethod`] that you are using.
///
/// Returns the length of the segment leading up to `segment_index`, returning
/// the live segment time if the split is not completed yet.
pub fn live_segment_time(
    timer: &Snapshot,
    segment_index: usize,
    method: TimingMethod,
) -> Option<TimeSpan> {
    live_segment_time_for_range(timer, segment_index, method)
}

/// Gets the length of the last segment range that leads up to the current
/// split, using the live segment time.
///
/// - `timer`: The current [`Timer`].
/// - `start_index`: The index of the first split in the range.
/// - `method`: The [`TimingMethod`] that you are using.
///
/// Returns the live length of the segment range.
pub fn live_segment_time_for_range(
    timer: &Snapshot,
    start_index: usize,
    method: TimingMethod,
) -> Option<TimeSpan> {
    segment_time(
        timer.run(),
        start_index,
        timer.current_time()[method]?,
        method,
    )
    .into()
}

/// Gets the amount of time lost or gained on a certain split.
///
/// - `timer`: The current [`Timer`].
/// - `segment_index`: The index of the split for which the delta is calculated.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The [`TimingMethod`] that you are using.
///
/// Returns the segment delta for a certain split, returning None if the split
/// is not completed yet.
pub fn previous_segment_delta(
    timer: &Timer,
    segment_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    previous_segment_delta_for_range(timer, segment_index, segment_index, comparison, method)
}

/// Gets the amount of time lost or gained over an inclusive range of segments.
///
/// - `timer`: The current [`Timer`].
/// - `start_index`: The index of the first split in the range.
/// - `end_index`: The index of the split for which the delta is calculated.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The [`TimingMethod`] that you are using.
///
/// Returns the segment delta for the range, returning None if the split is not
/// completed yet.
pub fn previous_segment_delta_for_range(
    timer: &Timer,
    start_index: usize,
    end_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    segment_delta(
        timer.run(),
        start_index,
        end_index,
        timer.run().segment(end_index).split_time()[method]?,
        comparison,
        method,
    )
}

/// Gets the amount of time lost or gained on a certain split, using the live
/// segment delta if the split is not completed yet.
///
/// - `timer`: The current [`Timer`].
/// - `segment_index`: The index of the split for which the delta is calculated.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The [`TimingMethod`] that you are using.
///
/// Returns the segment delta for a certain split, returning the live segment
/// delta if the split is not completed yet.
pub fn live_segment_delta(
    timer: &Snapshot,
    segment_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    live_segment_delta_for_range(timer, segment_index, segment_index, comparison, method)
}

/// Gets the amount of time lost or gained over an inclusive range of segments,
/// using the live segment delta if the split is not completed yet.
///
/// - `timer`: The current [`Timer`].
/// - `start_index`: The index of the first split in the range.
/// - `end_index`: The index of the split for which the delta is calculated.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The [`TimingMethod`] that you are using.
///
/// Returns the live segment delta for the range.
pub fn live_segment_delta_for_range(
    timer: &Snapshot,
    start_index: usize,
    end_index: usize,
    comparison: &str,
    method: TimingMethod,
) -> Option<TimeSpan> {
    segment_delta(
        timer.run(),
        start_index,
        end_index,
        timer.current_time()[method]?,
        comparison,
        method,
    )
}

/// Checks whether the live segment should now be shown.
///
/// - `timer`: The current [`Timer`].
/// - `split_delta`: Specifies whether to return a split delta rather than a
///   segment delta and to start showing the live segment once you are behind.
/// - `comparison`: The comparison that you are comparing with.
/// - `method`: The [`TimingMethod`] that you are using.
///
/// Returns the current live delta.
pub fn check_live_delta(
    timer: &Snapshot,
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
                && best_segment_delta.is_some_and(|d| d > TimeSpan::zero())
            || comparison_delta.is_some_and(|d| d > TimeSpan::zero())
        {
            return if split_delta {
                catch! { current_time? - current_split? }
            } else {
                comparison_delta
            };
        }
    }
    None
}

/// Chooses a split color from the
/// [`LayoutSettings`](crate::layout::LayoutSettings) based on the current run.
///
/// - `timer`: The current [`Timer`].
/// - `time_difference`: The delta that you want to find a color for.
/// - `segment_index`: The split number that is associated with this delta.
/// - `show_segment_deltas`: Can show ahead gaining and behind losing colors if
///   true.
/// - `show_best_segments`: Can show the best segment color if true.
/// - `comparison`: The comparison that you are comparing this delta to.
/// - `method`: The [`TimingMethod`] of this delta.
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
    split_color_for_range(
        timer,
        time_difference,
        segment_index..=segment_index,
        show_segment_deltas,
        show_best_segments,
        comparison,
        method,
    )
}

/// Chooses a split color for an inclusive segment range from the
/// [`LayoutSettings`](crate::layout::LayoutSettings) based on the current run.
pub fn split_color_for_range(
    timer: &Timer,
    time_difference: Option<TimeSpan>,
    segment_range: core::ops::RangeInclusive<usize>,
    show_segment_deltas: bool,
    show_best_segments: bool,
    comparison: &str,
    method: TimingMethod,
) -> SemanticColor {
    let start_index = *segment_range.start();
    let end_index = *segment_range.end();
    if show_best_segments && check_best_segment_for_range(timer, start_index, end_index, method) {
        SemanticColor::BestSegment
    } else if let Some(time_difference) = time_difference.filter(|t| t != &TimeSpan::zero()) {
        let last_delta = start_index
            .checked_sub(1)
            .and_then(|n| last_delta(timer.run(), n, comparison, method));
        if time_difference < TimeSpan::zero() {
            if show_segment_deltas && last_delta.is_some_and(|d| time_difference > d) {
                SemanticColor::AheadLosingTime
            } else {
                SemanticColor::AheadGainingTime
            }
        } else if show_segment_deltas && last_delta.is_some_and(|d| time_difference < d) {
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
/// - `timer`: The current [`Timer`].
/// - `segment_index`: The split to check.
/// - `method`: The [`TimingMethod`] to use.
///
/// Returns whether or not the indicated split is a Best Segment.
pub fn check_best_segment(timer: &Timer, segment_index: usize, method: TimingMethod) -> bool {
    check_best_segment_for_range(timer, segment_index, segment_index, method)
}

/// Calculates whether or not the Split Times for the indicated inclusive range
/// qualify as a Best Segment.
pub fn check_best_segment_for_range(
    timer: &Timer,
    start_index: usize,
    end_index: usize,
    method: TimingMethod,
) -> bool {
    if start_index == end_index {
        if timer.run().segment(end_index).split_time()[method].is_none() {
            return false;
        }

        let delta = previous_segment_delta(timer, end_index, best_segments::NAME, method);
        let current_segment = previous_segment_time(timer, end_index, method);
        let best_segment = timer.run().segment(end_index).best_segment_time()[method];
        return best_segment.is_none_or(|b| {
            current_segment.is_some_and(|c| c < b) || delta.is_some_and(|d| d < TimeSpan::zero())
        });
    }

    if timer.run().segment(end_index).split_time()[method].is_none() {
        return false;
    }

    let current_segment = previous_segment_time_for_range(timer, start_index, end_index, method);
    let best_segment = comparison_segment_time_for_range(
        timer.run(),
        start_index,
        end_index,
        best_segments::NAME,
        method,
    );
    best_segment.is_none_or(|b| current_segment.is_some_and(|c| c < b))
}
