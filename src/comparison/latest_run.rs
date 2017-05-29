use {Segment, Attempt, TimingMethod, TimeSpan};
use super::ComparisonGenerator;

#[derive(Copy, Clone, Debug)]
pub struct LatestRun;

pub const NAME: &'static str = "Latest Run";

fn generate(segments: &mut [Segment], method: TimingMethod) {
    let mut attempt_id = None;
    for segment in segments.iter_mut().rev() {
        if let Some(max_index) = segment.segment_history().try_get_max_index() {
            attempt_id = Some(max_index);
            break;
        }
    }

    if let Some(attempt_id) = attempt_id {
        let mut remaining_segments = segments.iter_mut();

        let mut total_time = TimeSpan::zero();
        while let Some(segment) = remaining_segments.next() {
            let segment_time = segment
                .segment_history()
                .get(attempt_id)
                .map(|t| t[method]);

            let split_time = match segment_time {
                Some(Some(segment_time)) => {
                    total_time += segment_time;
                    Some(total_time)
                }
                Some(None) => None,
                None => {
                    segment.comparison_mut(NAME)[method] = None;
                    break;
                }
            };

            segment.comparison_mut(NAME)[method] = split_time;
        }

        for segment in remaining_segments {
            segment.comparison_mut(NAME)[method] = None;
        }
    }
}

impl ComparisonGenerator for LatestRun {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        generate(segments, TimingMethod::RealTime);
        generate(segments, TimingMethod::GameTime);
    }
}
