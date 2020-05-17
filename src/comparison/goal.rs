//! Defines functions for generating a goal comparison based on a goal time provided.
//! The comparison's times are automatically balanced based on the runner's
//! history such that it roughly represents what split times for the goal time
//! would roughly look like. This does not define a Comparison Generator. The
//! Balanced PB comparison however is based on this, which uses the Personal
//! Best as a goal time to balance the mistakes that happened in the Personal Best.

use crate::{analysis::SkillCurve, Run, Segment, Time, TimeSpan, TimingMethod};

/// The default name of the goal comparison.
pub const NAME: &str = "Goal";

// FIXME: Possibly move this into the analysis module.
pub(crate) fn determine_percentile(
    offset: TimeSpan,
    segments: &[Segment],
    method: TimingMethod,
    goal_time: Option<TimeSpan>,
    skill_curve: &mut SkillCurve,
) -> f64 {
    skill_curve.for_segments(segments, method);

    // Depending on whether we have a goal time or not, we use that goal time
    // or try to determine a personal best split time that we use for the goal
    // time. In that case we may need to limit the slice again to the last split
    // that actually has a split time we can work with.
    let goal_time = if let Some(goal_time) = goal_time {
        goal_time
    } else {
        let (new_len, goal_time) = segments[..skill_curve.len()]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, s)| s.personal_best_split_time()[method].map(|t| (i + 1, t)))
            .unwrap_or_default();
        skill_curve.truncate(new_len);
        goal_time
    };

    skill_curve.find_percentile_for_time(offset, goal_time)
}

pub(super) fn generate_for_timing_method_with_buf(
    segments: &mut [Segment],
    method: TimingMethod,
    goal_time: Option<TimeSpan>,
    comparison: &str,
    skill_curve: &mut SkillCurve,
) {
    let percentile =
        determine_percentile(TimeSpan::zero(), segments, method, goal_time, skill_curve);

    let mut segments = segments.iter_mut();
    for (segment, val) in segments
        .by_ref()
        .zip(skill_curve.iter_split_times_at_percentile(percentile, TimeSpan::zero()))
    {
        segment.comparison_mut(comparison)[method] = Some(val);
    }
    for segment in segments {
        segment.comparison_mut(comparison)[method] = None;
    }
}

/// Populates the segments with a goal comparison for the timing method
/// specified. Every other timing method is left untouched. The segment history
/// is used to generate comparison times such that they end up with the goal
/// time specified. The values are stored in the comparison with the name
/// provided. Only the range between the sum of the best segments and the sum of
/// the worst segments is supported. Every other goal time is capped within that
/// range.
pub fn generate_for_timing_method(
    segments: &mut [Segment],
    method: TimingMethod,
    goal_time: TimeSpan,
    comparison: &str,
) {
    let mut skill_curve = SkillCurve::new();

    generate_for_timing_method_with_buf(
        segments,
        method,
        Some(goal_time),
        comparison,
        &mut skill_curve,
    );
}

/// Populates the segments with a goal comparison. The segment history is used
/// to generate comparison times such that they end up with the goal time
/// specified. The values are stored in the comparison with the name provided.
/// Only the range between the sum of the best segments and the sum of the worst
/// segments is supported. Every other goal time is capped within that range.
pub fn generate(segments: &mut [Segment], goal_time: Time, comparison: &str) {
    let mut skill_curve = SkillCurve::new();

    if let Some(real_time) = goal_time.real_time {
        generate_for_timing_method_with_buf(
            segments,
            TimingMethod::RealTime,
            Some(real_time),
            comparison,
            &mut skill_curve,
        );
    } else {
        for segment in &mut *segments {
            segment.comparison_mut(comparison).real_time = None;
        }
    }

    if let Some(game_time) = goal_time.game_time {
        generate_for_timing_method_with_buf(
            segments,
            TimingMethod::GameTime,
            Some(game_time),
            comparison,
            &mut skill_curve,
        );
    } else {
        for segment in &mut *segments {
            segment.comparison_mut(comparison).game_time = None;
        }
    }
}

fn round_up(value: i64, factor: i64) -> i64 {
    (value + factor - 1) / factor * factor
}

fn nice_goal_time(precise_goal_time: TimeSpan, pb: TimeSpan) -> TimeSpan {
    let total_seconds = precise_goal_time.total_seconds() as i64;
    let pb_seconds = pb.total_seconds() as i64;
    for factor in [60 * 60, 60 * 15, 60 * 5, 60, 15, 5].iter().copied() {
        let goal_seconds = round_up(total_seconds, factor);
        if goal_seconds < pb_seconds {
            return TimeSpan::from_seconds(goal_seconds as f64);
        }
    }
    precise_goal_time
}

pub fn suggest_goal_time(run: &Run, method: TimingMethod) -> Option<TimeSpan> {
    // let mut skill_curve = SkillCurve::new();

    // let percentile = determine_percentile(
    //     TimeSpan::zero(),
    //     run.segments(),
    //     method,
    //     None,
    //     &mut skill_curve,
    // );

    // let goal_time = skill_curve
    //     .iter_split_times_at_percentile(0.85 * percentile, TimeSpan::zero())
    //     .last()?;

    // Some(nice_goal_time(goal_time, pb))
    todo!()
}
