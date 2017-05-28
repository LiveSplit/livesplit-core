use {Segment, Attempt, TimingMethod, TimeSpan};
use super::ComparisonGenerator;

#[derive(Copy, Clone, Debug)]
pub struct LastFinishedRun;

pub const NAME: &'static str = "Last Finished Run";

fn generate(segments: &mut [Segment], attempts: &[Attempt], method: TimingMethod) {
    let attempt = attempts
        .iter()
        .rev()
        .find(|a| a.time()[method].is_some());

    if let Some(attempt) = attempt {
        let id = attempt.index();
        let mut total_time = TimeSpan::zero();
        for segment in segments {
            let segment_time = segment
                .segment_history()
                .get(id)
                .and_then(|t| t[method]);

            let split_time = if let Some(segment_time) = segment_time {
                total_time += segment_time;
                Some(total_time)
            } else {
                None
            };

            segment.comparison_mut(NAME)[method] = split_time;
        }
    } else {
        for segment in segments {
            segment.comparison_mut(NAME)[method] = None;
        }
    }
}

impl ComparisonGenerator for LastFinishedRun {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], attempts: &[Attempt]) {
        generate(segments, attempts, TimingMethod::RealTime);
        generate(segments, attempts, TimingMethod::GameTime);
    }
}
