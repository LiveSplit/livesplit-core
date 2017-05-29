use super::ComparisonGenerator;
use {Attempt, Segment, TimeSpan, TimingMethod};

#[derive(Copy, Clone, Debug)]
pub struct AverageSegments;

pub const NAME: &'static str = "Average Segments";

const WEIGHT: f64 = 0.75;

fn calculate_weight(index: usize, count: usize) -> f64 {
    WEIGHT.powi((count - index - 1) as i32)
}

fn calculate_average(times: &[(i32, TimeSpan)]) -> TimeSpan {
    let (mut total_weights, mut total_time) = (0.0, 0.0);

    for (i, &(_, time)) in times.iter().enumerate() {
        let weight = calculate_weight(i, times.len());
        total_weights += weight;
        total_time += weight * time.total_seconds();
    }

    TimeSpan::from_seconds(total_time / total_weights)
}

fn generate(buf: &mut Vec<(i32, TimeSpan)>, segments: &mut [Segment], method: TimingMethod) {
    let mut total_time = Some(TimeSpan::zero());

    for i in 0..segments.len() {
        if total_time.is_some() {
            buf.clear();

            for &(id, time) in segments[i].segment_history().iter_actual_runs() {
                if let Some(time) = time[method] {
                    let keep = i.checked_sub(1)
                        .and_then(|i| {
                                      segments[i]
                                          .segment_history()
                                          .get(id)
                                          .map(|t| t[method].is_some())
                                  })
                        .unwrap_or(true);

                    if keep {
                        buf.push((id, time));
                    }
                }
            }

            if buf.is_empty() {
                total_time = None;
            }
            if let Some(ref mut total_time) = total_time {
                *total_time = *total_time + calculate_average(&buf);
            }
        }
        segments[i].comparison_mut(NAME)[method] = total_time;
    }
}

impl ComparisonGenerator for AverageSegments {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        let mut times = Vec::new();

        generate(&mut times, segments, TimingMethod::RealTime);
        generate(&mut times, segments, TimingMethod::GameTime);
    }
}
