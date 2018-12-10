//! Provides functions for calculating how much time save there is for either
//! single segments or the remainder of an active attempt. This information is
//! based on the best segments. Considering the best segments don't represent
//! theoretically perfect segment times, this information is only an
//! approximation of how much time can actually be saved.

use crate::{analysis, TimeSpan, Timer};

/// Calculates how much time could be saved on the given segment with the given
/// comparison. This information is based on the best segments. Considering the
/// best segments don't represent theoretically perfect segment times, this
/// information is only an approximation of how much time can actually be saved.
/// If the parameter `live` is set to `true`, then the segment time of the
/// current attempt is used if it gets longer than the segment time of the
/// segment the possible time save is calculated for. So the possible time save
/// shrinks towards zero as time goes on. The time returned by this function can
/// never be below zero.
pub fn calculate(
    timer: &Timer,
    segment_index: usize,
    comparison: &str,
    live: bool,
) -> Option<TimeSpan> {
    let segments = timer.run().segments();
    let method = timer.current_timing_method();
    let mut prev_time = TimeSpan::zero();
    let segment = timer.run().segment(segment_index);
    let mut best_segments = segment.best_segment_time()[method];

    for segment in segments[..segment_index].iter().rev() {
        if let Some(best_segments) = &mut best_segments {
            if let Some(split_time) = segment.comparison(comparison)[method] {
                prev_time = split_time;
                break;
            } else if let Some(best_segment) = segment.best_segment_time()[method] {
                *best_segments += best_segment;
            }
        } else {
            break;
        }
    }

    catch! {
        let mut time = segment.comparison(comparison)[method]? - best_segments? - prev_time;

        catch! {
            if live && timer.current_split_index()? == segment_index {
                let segment_delta = analysis::live_segment_delta(
                    timer,
                    segment_index,
                    comparison,
                    method,
                )?;
                let segment_delta = TimeSpan::zero() - segment_delta;
                if segment_delta < time {
                    time = segment_delta;
                }
            };
        };

        if time < TimeSpan::zero() {
            TimeSpan::zero()
        } else {
            time
        }
    }
}

/// Calculates how much time could be saved on the remainder of the run with the
/// given comparison. This information is based on the best segments.
/// Considering the best segments don't represent theoretically perfect segment
/// times, this information is only an approximation of how much time can
/// actually be saved. This information is always live, so the total possible
/// time save will shrink towards zero throughout the run and when time is lost
/// on a segment. The time returned by this function can never be below zero.
pub fn calculate_total(timer: &Timer, segment_index: usize, comparison: &str) -> TimeSpan {
    let mut total = TimeSpan::zero();

    for index in segment_index..timer.run().len() {
        if let Some(time_save) = calculate(timer, index, comparison, true) {
            total += time_save;
        }
    }

    total
}
