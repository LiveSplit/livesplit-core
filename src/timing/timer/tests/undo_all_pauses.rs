use super::run;
use crate::{Time, TimeSpan, Timer, TimerPhase, event::Event};

fn timer() -> Timer {
    let mut run = run();
    // Keep the real time positive while simulating pauses without sleeping.
    run.set_offset(TimeSpan::from_seconds(100.0));
    let mut timer = Timer::new(run).unwrap();
    timer.start().unwrap();
    timer
}

fn set_pause_time(timer: &mut Timer, seconds: f64) {
    let attempt = timer.active_attempt.as_mut().unwrap();
    attempt.adjusted_offset = attempt.original_offset - TimeSpan::from_seconds(seconds);
}

#[track_caller]
fn assert_adjusted(
    timer: &Timer,
    index: usize,
    before: Time,
    real_time_seconds: f64,
    game_time_seconds: f64,
) {
    let after = timer.run().segment(index).split_time();
    assert_eq!(
        after.real_time,
        before
            .real_time
            .map(|time| time + TimeSpan::from_seconds(real_time_seconds))
    );
    assert_eq!(
        after.game_time,
        before
            .game_time
            .map(|time| time + TimeSpan::from_seconds(game_time_seconds))
    );
}

#[test]
fn adjusts_each_split_by_its_accumulated_pauses() {
    for (initialize_game_time, explicitly_set_game_time) in
        [(false, false), (true, false), (true, true)]
    {
        let mut timer = timer();
        if initialize_game_time {
            timer.initialize_game_time().unwrap();
            // Auto splitters can use only these calls to exclude loading times.
            timer.pause_game_time().unwrap();
            timer.resume_game_time().unwrap();
        }
        if explicitly_set_game_time {
            timer.pause_game_time().unwrap();
        }

        let mut splits = Vec::new();
        for (index, seconds) in [0.0, 2.0, 5.0].into_iter().enumerate() {
            set_pause_time(&mut timer, seconds);
            if explicitly_set_game_time {
                timer
                    .set_game_time(TimeSpan::from_seconds(10.0 * (index + 1) as f64))
                    .unwrap();
            }
            timer.split().unwrap();
            splits.push(timer.run().segment(index).split_time());
        }

        timer.mark_as_unmodified();
        assert_eq!(timer.undo_all_pauses().unwrap(), Event::PausesUndone);
        assert!(timer.run().has_been_modified());
        assert_eq!(timer.current_phase(), TimerPhase::Ended);
        assert_eq!(timer.get_pause_time(), None);
        for (index, seconds) in [0.0, 2.0, 5.0].into_iter().enumerate() {
            let game_time_seconds = if explicitly_set_game_time {
                0.0
            } else {
                seconds
            };
            assert_adjusted(&timer, index, splits[index], seconds, game_time_seconds);
        }
        let final_time = timer.run().segment(2).split_time();
        assert_eq!(timer.snapshot().current_time(), final_time);

        timer.mark_as_unmodified();
        timer.undo_all_pauses().unwrap();
        assert!(!timer.run().has_been_modified());
        for (index, seconds) in [0.0, 2.0, 5.0].into_iter().enumerate() {
            let game_time_seconds = if explicitly_set_game_time {
                0.0
            } else {
                seconds
            };
            assert_adjusted(&timer, index, splits[index], seconds, game_time_seconds);
        }

        timer.reset(true).unwrap();
        assert_eq!(
            timer.run().attempt_history().last().unwrap().time(),
            final_time
        );
        for (index, seconds) in [0.0, 2.0, 5.0].into_iter().enumerate() {
            let game_time_seconds = if explicitly_set_game_time {
                0.0
            } else {
                seconds
            };
            assert_eq!(
                timer.run().segment(index).personal_best_split_time(),
                Time {
                    real_time: splits[index]
                        .real_time
                        .map(|time| time + TimeSpan::from_seconds(seconds)),
                    game_time: splits[index]
                        .game_time
                        .map(|time| time + TimeSpan::from_seconds(game_time_seconds)),
                }
            );
        }
    }
}

#[test]
fn adjusts_existing_splits_while_running_and_after_more_pauses() {
    let mut timer = timer();
    timer.initialize_game_time().unwrap();
    set_pause_time(&mut timer, 2.0);
    timer.split().unwrap();
    let first = timer.run().segment(0).split_time();
    let loading_times = timer.loading_times();

    timer.undo_all_pauses().unwrap();
    assert_adjusted(&timer, 0, first, 2.0, 2.0);
    assert_eq!(timer.run().segment(1).split_time(), Time::new());
    assert_eq!(timer.run().segment(2).split_time(), Time::new());
    assert_eq!(timer.current_phase(), TimerPhase::Running);
    assert_eq!(timer.get_pause_time(), None);
    assert_eq!(timer.loading_times(), loading_times);

    set_pause_time(&mut timer, 3.0);
    timer.split().unwrap();
    let second = timer.run().segment(1).split_time();
    timer.undo_all_pauses().unwrap();
    assert_adjusted(&timer, 0, first, 2.0, 2.0);
    assert_adjusted(&timer, 1, second, 3.0, 3.0);

    timer.split().unwrap();
    let third = timer.run().segment(2).split_time();
    timer.undo_all_pauses().unwrap();
    assert_adjusted(&timer, 0, first, 2.0, 2.0);
    assert_adjusted(&timer, 1, second, 3.0, 3.0);
    assert_adjusted(&timer, 2, third, 0.0, 0.0);
}

#[test]
fn resumes_without_applying_the_current_pause_to_earlier_splits() {
    let mut timer = timer();
    timer.pause_game_time().unwrap();
    timer.set_game_time(TimeSpan::from_seconds(10.0)).unwrap();
    set_pause_time(&mut timer, 2.0);
    timer.split().unwrap();
    let first = timer.run().segment(0).split_time();

    set_pause_time(&mut timer, 5.0);
    timer.pause().unwrap();
    let game_time = timer.snapshot().current_time().game_time;
    assert_eq!(
        timer.undo_all_pauses().unwrap(),
        Event::PausesUndoneAndResumed
    );
    assert_eq!(timer.current_phase(), TimerPhase::Running);
    assert_eq!(timer.get_pause_time(), None);
    assert_eq!(timer.snapshot().current_time().game_time, game_time);
    assert_adjusted(&timer, 0, first, 2.0, 0.0);
}

#[test]
fn handles_skipped_and_undone_splits() {
    let mut timer = timer();
    set_pause_time(&mut timer, 2.0);
    timer.split().unwrap();
    let first = timer.run().segment(0).split_time();

    set_pause_time(&mut timer, 5.0);
    timer.split().unwrap();
    timer.undo_split().unwrap();
    timer.skip_split().unwrap();
    set_pause_time(&mut timer, 7.0);
    timer.split().unwrap();

    // Reopen a finished attempt and replace its final split as well.
    timer.undo_split().unwrap();
    set_pause_time(&mut timer, 9.0);
    timer.split().unwrap();
    let third = timer.run().segment(2).split_time();

    timer.undo_all_pauses().unwrap();
    assert_adjusted(&timer, 0, first, 2.0, 0.0);
    assert_eq!(timer.run().segment(1).split_time(), Time::new());
    assert_adjusted(&timer, 2, third, 9.0, 0.0);
}

#[test]
fn discards_pause_times_when_starting_a_new_attempt() {
    let mut timer = timer();
    timer.set_game_time(TimeSpan::from_seconds(10.0)).unwrap();
    set_pause_time(&mut timer, 2.0);
    timer.split().unwrap();
    timer.reset(false).unwrap();
    timer.start().unwrap();
    timer.initialize_game_time().unwrap();
    set_pause_time(&mut timer, 3.0);
    timer.split().unwrap();
    let first = timer.run().segment(0).split_time();

    timer.undo_all_pauses().unwrap();
    assert_adjusted(&timer, 0, first, 3.0, 3.0);

    timer.mark_as_unmodified();
    timer.undo_all_pauses().unwrap();
    assert!(!timer.run().has_been_modified());
    assert_adjusted(&timer, 0, first, 3.0, 3.0);
}

#[test]
fn setting_game_time_preserves_all_game_time_splits() {
    let mut timer = timer();
    timer.initialize_game_time().unwrap();
    let mut splits = Vec::new();
    for (index, seconds) in [2.0, 5.0, 7.0].into_iter().enumerate() {
        set_pause_time(&mut timer, seconds);
        if index == 1 {
            timer.set_game_time(TimeSpan::from_seconds(110.0)).unwrap();
        }
        timer.split().unwrap();
        splits.push(timer.run().segment(index).split_time());
    }

    timer.undo_all_pauses().unwrap();
    for (index, seconds) in [2.0, 5.0, 7.0].into_iter().enumerate() {
        assert_adjusted(&timer, index, splits[index], seconds, 0.0);
    }
}

#[test]
fn reinitializing_game_time_keeps_explicit_game_time_protected() {
    let mut timer = timer();
    timer.set_game_time(TimeSpan::from_seconds(10.0)).unwrap();
    set_pause_time(&mut timer, 2.0);
    timer.split().unwrap();
    let first = timer.run().segment(0).split_time();

    timer.deinitialize_game_time();
    timer.initialize_game_time().unwrap();
    set_pause_time(&mut timer, 5.0);
    timer.split().unwrap();
    let second = timer.run().segment(1).split_time();
    let loading_times = timer.loading_times();

    timer.undo_all_pauses().unwrap();
    assert_adjusted(&timer, 0, first, 2.0, 0.0);
    assert_adjusted(&timer, 1, second, 5.0, 0.0);
    assert_eq!(
        timer.loading_times(),
        loading_times + TimeSpan::from_seconds(5.0)
    );
}

#[test]
fn adjusts_paused_derived_game_time_without_resuming_the_game_timer() {
    let mut timer = timer();
    timer.initialize_game_time().unwrap();
    set_pause_time(&mut timer, 2.0);
    timer.split().unwrap();
    let first = timer.run().segment(0).split_time();

    timer.pause_game_time().unwrap();
    set_pause_time(&mut timer, 5.0);
    let game_time = timer.snapshot().current_time().game_time.unwrap();
    timer.undo_all_pauses().unwrap();

    assert_adjusted(&timer, 0, first, 2.0, 2.0);
    assert_eq!(
        timer.snapshot().current_time().game_time,
        Some(game_time + TimeSpan::from_seconds(5.0))
    );
    assert!(timer.is_game_time_paused());
    timer.mark_as_unmodified();
    timer.undo_all_pauses().unwrap();
    assert!(!timer.run().has_been_modified());
    assert_eq!(
        timer.snapshot().current_time().game_time,
        Some(game_time + TimeSpan::from_seconds(5.0))
    );
}
