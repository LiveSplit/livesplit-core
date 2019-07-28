use crate::{comparison, Run, Segment, TimeSpan, Timer, TimingMethod};

#[cfg(test)]
mod tests;

fn calculate(segments: &[Segment], method: TimingMethod, offset: TimeSpan) -> f64 {
    if segments
        .last()
        .and_then(|s| s.personal_best_split_time()[method])
        .is_none()
    {
        // If there is no PB time, then it's always a 100% chance.
        return 1.0;
    }

    let mut all_weighted_segment_times = vec![Vec::new(); segments.len()];
    let mut time_span_buf = Vec::with_capacity(segments.len());

    comparison::goal::determine_percentile(
        offset,
        segments,
        method,
        None,
        &mut time_span_buf,
        &mut all_weighted_segment_times,
    )
}

pub fn for_run(run: &Run, method: TimingMethod) -> f64 {
    calculate(run.segments(), method, TimeSpan::zero())
}

pub fn for_timer(timer: &Timer) -> f64 {
    let method = timer.current_timing_method();
    let all_segments = timer.run().segments();

    let live_delta = super::check_live_delta(timer, true, comparison::personal_best::NAME, method);

    let (segments, current_time) = if live_delta.is_some() {
        // If there is a live delta, act as if we did just split.
        (
            &all_segments[timer.current_split_index().unwrap() + 1..],
            timer.current_time()[method].unwrap_or_default(),
        )
    } else if let Some((index, time)) = all_segments
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, s)| Some((i, s.split_time()[method]?)))
    {
        // Otherwise fall back to the the last split that we did split.
        (&all_segments[index + 1..], time)
    } else {
        // Otherwise fall back to all segments with a timer that didn't really
        // start.
        (all_segments, TimeSpan::zero())
    };

    // If there are no more segments, which can be because either there is a
    // live delta and we are on the final split, or if we actually did split the
    // final split, then we want to simply compare the current time to the PB
    // time and then either return 100% or 0% based on whether our new time is a
    // PB or not.
    if segments.is_empty() {
        let beat_pb = all_segments
            .last()
            .and_then(|s| s.personal_best_split_time()[method])
            .map_or(true, |pb| current_time < pb);
        if beat_pb {
            1.0
        } else {
            0.0
        }
    } else {
        calculate(segments, method, current_time)
    }
}
