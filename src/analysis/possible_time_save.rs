use {analysis, TimeSpan, Timer};

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
        if let Some(ref mut best_segments) = best_segments {
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

pub fn calculate_total(timer: &Timer, segment_index: usize, comparison: &str) -> TimeSpan {
    let mut total = TimeSpan::zero();

    for index in segment_index..timer.run().len() {
        if let Some(time_save) = calculate(timer, index, comparison, true) {
            total += time_save;
        }
    }

    total
}
