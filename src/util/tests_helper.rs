#![allow(dead_code)]

use crate::{Run, Segment, TimeSpan, Timer, TimingMethod};

#[track_caller]
pub fn create_run(names: &[&str]) -> Run {
    let mut run = Run::new();
    for &name in names {
        run.push_segment(Segment::new(name));
    }
    run
}

#[track_caller]
pub fn create_timer(names: &[&str]) -> Timer {
    Timer::new(create_run(names)).unwrap()
}

#[track_caller]
pub fn start_run(timer: &mut Timer) {
    timer.set_current_timing_method(TimingMethod::GameTime);
    timer.start().unwrap();
    timer.initialize_game_time().unwrap();
    timer.pause_game_time().unwrap();
    timer.set_game_time(TimeSpan::zero()).unwrap();
}

#[track_caller]
pub fn run_with_splits(timer: &mut Timer, splits: &[f64]) {
    start_run(timer);

    for &split in splits {
        timer.set_game_time(TimeSpan::from_seconds(split)).unwrap();
        timer.split().unwrap();
    }

    timer.reset(true).unwrap();
}

/// Same as run_with_splits_opt, but progresses an already active attempt and
/// doesn't reset it. Useful for checking intermediate states.
#[track_caller]
pub fn make_progress_run_with_splits_opt(timer: &mut Timer, splits: &[Option<f64>]) {
    for &split in splits {
        if let Some(split) = split {
            timer.set_game_time(TimeSpan::from_seconds(split)).unwrap();
            timer.split().unwrap();
        } else {
            timer.skip_split().unwrap();
        }
    }
}

#[track_caller]
pub fn run_with_splits_opt(timer: &mut Timer, splits: &[Option<f64>]) {
    start_run(timer);
    make_progress_run_with_splits_opt(timer, splits);
    timer.reset(true).unwrap();
}

#[track_caller]
pub fn span(seconds: f64) -> TimeSpan {
    TimeSpan::from_seconds(seconds)
}
