use {TimeSpan, Run, TimingMethod};
use super::{track_branch, track_current_run, track_personal_best_run};

fn populate_prediction(prediction: &mut Option<TimeSpan>, predicted_time: Option<TimeSpan>) {
    if let Some(predicted_time) = predicted_time {
        if prediction.map_or(true, |t| predicted_time < t) {
            *prediction = Some(predicted_time);
        }
    }
}

fn populate_predictions(run: &Run,
                        current_time: Option<TimeSpan>,
                        segment_index: usize,
                        predictions: &mut [Option<TimeSpan>],
                        simple_calculation: bool,
                        use_current_run: bool,
                        method: TimingMethod) {
    if let Some(current_time) = current_time {
        populate_prediction(&mut predictions[segment_index + 1],
                            run.segment(segment_index).best_segment_time()[method]
                                .map(|t| t + current_time));
        if !simple_calculation {
            for (&null_segment_index, _) in
                run.segment(segment_index)
                    .segment_history()
                    .iter()
                    .filter(|&(_, t)| t[method].is_none()) {

                let should_track_branch = segment_index.checked_sub(1)
                    .and_then(|previous_index| {
                        run.segment(previous_index).segment_history().get(null_segment_index)
                    })
                    .map_or(true, |segment_time| segment_time[method].is_some());

                if should_track_branch {
                    let (index, time) = track_branch(run,
                                                     Some(current_time),
                                                     segment_index + 1,
                                                     null_segment_index,
                                                     method);
                    populate_prediction(&mut predictions[index], time[method]);
                }
            }
        }
        if use_current_run {
            let (index, time) = track_current_run(run, Some(current_time), segment_index, method);
            populate_prediction(&mut predictions[index], time[method]);
        }
        let (index, time) = track_personal_best_run(run, Some(current_time), segment_index, method);
        populate_prediction(&mut predictions[index], time[method]);
    }
}

#[allow(needless_range_loop, unknown_lints)]
pub fn calculate(run: &Run,
                 start_index: usize,
                 end_index: usize, // Exclusive
                 predictions: &mut [Option<TimeSpan>],
                 simple_calculation: bool,
                 use_current_run: bool,
                 method: TimingMethod)
                 -> Option<TimeSpan> {
    predictions[start_index] = Some(TimeSpan::zero());
    for segment_index in start_index..end_index {
        let current_time = predictions[segment_index];
        populate_predictions(run,
                             current_time,
                             segment_index,
                             predictions,
                             simple_calculation,
                             use_current_run,
                             method);
    }
    predictions[end_index]
}
