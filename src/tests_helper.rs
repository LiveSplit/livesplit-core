use crate::{Run, Segment, TimeSpan, Timer, TimingMethod};

pub fn create_run(names: &[&str]) -> Run {
    let mut run = Run::new();
    for &name in names {
        run.push_segment(Segment::new(name));
    }
    run
}

pub fn create_timer(names: &[&str]) -> Timer {
    Timer::new(create_run(names)).unwrap()
}

pub fn start_run(timer: &mut Timer) {
    timer.set_current_timing_method(TimingMethod::GameTime);
    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();
    timer.set_game_time(TimeSpan::zero());
}

pub fn run_with_splits(timer: &mut Timer, splits: &[f64]) {
    start_run(timer);

    for &split in splits {
        timer.set_game_time(TimeSpan::from_seconds(split));
        timer.split();
    }

    timer.reset(true);
}

/// Same as run_with_splits_opt, but progresses an already active attempt and
/// doesn't reset it. Useful for checking intermediate states.
pub fn make_progress_run_with_splits_opt(timer: &mut Timer, splits: &[Option<f64>]) {
    for &split in splits {
        if let Some(split) = split {
            timer.set_game_time(TimeSpan::from_seconds(split));
            timer.split();
        } else {
            timer.skip_split();
        }
    }
}

pub fn run_with_splits_opt(timer: &mut Timer, splits: &[Option<f64>]) {
    start_run(timer);
    make_progress_run_with_splits_opt(timer, splits);
    timer.reset(true);
}

pub fn span(seconds: f64) -> TimeSpan {
    TimeSpan::from_seconds(seconds)
}
