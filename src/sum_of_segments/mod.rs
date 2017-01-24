pub mod best;
pub mod worst;

use {Segment, TimeSpan, TimingMethod, Time};

pub fn calculate_best(segments: &[Segment],
                      simple_calculation: bool,
                      use_current_run: bool,
                      method: TimingMethod)
                      -> Option<TimeSpan> {
    let mut predictions = Vec::with_capacity(segments.len() + 1);
    predictions.resize(segments.len() + 1, None);
    best::calculate(segments,
                    0,
                    segments.len(),
                    &mut predictions,
                    simple_calculation,
                    use_current_run,
                    method)
}

pub fn calculate_worst(segments: &[Segment],
                       use_current_run: bool,
                       method: TimingMethod)
                       -> Option<TimeSpan> {
    let mut predictions = Vec::with_capacity(segments.len() + 1);
    predictions.resize(segments.len() + 1, None);
    worst::calculate(segments,
                     0,
                     segments.len(),
                     &mut predictions,
                     use_current_run,
                     method)
}

fn track_current_run(segments: &[Segment],
                     current_time: Option<TimeSpan>,
                     segment_index: usize,
                     method: TimingMethod)
                     -> (usize, Time) {
    if let Some(first_split_time) =
        segment_index.checked_sub(1)
            .map_or(Some(TimeSpan::zero()), |i| segments[i].split_time()[method]) {
        for (segment_index, segment) in segments.iter().enumerate().skip(segment_index) {
            let second_split_time = segment.split_time()[method];
            if let Some(second_split_time) = second_split_time {
                return (segment_index + 1,
                        Time::new().with_timing_method(method,
                                                       current_time.map(|t| {
                                                           second_split_time - first_split_time + t
                                                       })));
            }
        }
    }
    (0, Time::default())
}

fn track_personal_best_run(segments: &[Segment],
                           current_time: Option<TimeSpan>,
                           segment_index: usize,
                           method: TimingMethod)
                           -> (usize, Time) {
    if let Some(first_split_time) =
        segment_index.checked_sub(1).map_or(Some(TimeSpan::zero()),
                                            |i| segments[i].personal_best_split_time()[method]) {
        for (segment_index, segment) in segments.iter().enumerate().skip(segment_index) {
            let second_split_time = segment.personal_best_split_time()[method];
            if let Some(second_split_time) = second_split_time {
                return (segment_index + 1,
                        Time::new().with_timing_method(method,
                                                       current_time.map(|t| {
                                                           second_split_time - first_split_time + t
                                                       })));
            }
        }
    }
    (0, Time::default())
}

fn track_branch(segments: &[Segment],
                current_time: Option<TimeSpan>,
                segment_index: usize,
                run_index: i32,
                method: TimingMethod)
                -> (usize, Time) {
    for (segment_index, segment) in segments.iter().enumerate().skip(segment_index) {
        if let Some(cur_time) = segment.segment_history().get(run_index) {
            if let Some(cur_time) = cur_time[method] {
                return (segment_index + 1,
                        Time::new().with_timing_method(method, current_time.map(|t| cur_time + t)));
            }
        } else {
            break;
        }
    }
    (0, Time::default())
}
