use crate::{
    TimeSpan, Timer, comparison,
    event::{Error, Event},
};

use super::{run, timer};

mod start {
    use super::*;

    #[test]
    fn new_run_works() {
        let mut timer = timer();

        let event = timer.start().unwrap();

        assert_eq!(event, Event::Started);
    }

    #[test]
    fn two_runs_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        let error = timer.start().unwrap_err();

        assert_eq!(error, Error::RunAlreadyInProgress);
    }
}

mod split {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.split().unwrap();

        assert_eq!(event, Event::Splitted);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.split().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn finished_run_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let error = timer.split().unwrap_err();

        assert_eq!(error, Error::RunFinished);
    }

    #[test]
    fn before_timer_is_at_0_fails() {
        let mut run = run();
        run.set_offset(TimeSpan::from_seconds(-1000.0));
        let mut timer = Timer::new(run).unwrap();

        timer.start().unwrap();
        let error = timer.split().unwrap_err();

        assert_eq!(error, Error::NegativeTime);
    }

    #[test]
    fn final_split_finishes_the_run() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.split().unwrap();

        assert_eq!(event, Event::Finished);
    }

    #[test]
    fn while_paused_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let error = timer.split().unwrap_err();

        assert_eq!(error, Error::TimerPaused);
    }
}

mod split_or_start {
    use super::*;

    #[test]
    fn starting_new_run_works() {
        let mut timer = timer();

        let event = timer.split_or_start().unwrap();

        assert_eq!(event, Event::Started);
    }

    #[test]
    fn splitting_works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.split_or_start().unwrap();

        assert_eq!(event, Event::Splitted);
    }

    #[test]
    fn finished_run_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let error = timer.split_or_start().unwrap_err();

        assert_eq!(error, Error::RunFinished);
    }

    #[test]
    fn before_timer_is_at_0_fails() {
        let mut run = run();
        run.set_offset(TimeSpan::from_seconds(-1000.0));
        let mut timer = Timer::new(run).unwrap();

        timer.start().unwrap();
        let error = timer.split_or_start().unwrap_err();

        assert_eq!(error, Error::NegativeTime);
    }

    #[test]
    fn final_split_finishes_the_run() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.split_or_start().unwrap();

        assert_eq!(event, Event::Finished);
    }

    #[test]
    fn while_paused_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let error = timer.split_or_start().unwrap_err();

        assert_eq!(error, Error::TimerPaused);
    }
}

mod reset {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.reset(true).unwrap();

        assert_eq!(event, Event::Reset);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.reset(true).unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn finished_run_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.reset(true).unwrap();

        assert_eq!(event, Event::Reset);
    }

    #[test]
    fn while_paused_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let event = timer.reset(true).unwrap();

        assert_eq!(event, Event::Reset);
    }
}

mod reset_and_set_attempt_as_pb {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.reset_and_set_attempt_as_pb().unwrap();

        assert_eq!(event, Event::Reset);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.reset_and_set_attempt_as_pb().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn finished_run_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.reset_and_set_attempt_as_pb().unwrap();

        assert_eq!(event, Event::Reset);
    }

    #[test]
    fn while_paused_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let event = timer.reset_and_set_attempt_as_pb().unwrap();

        assert_eq!(event, Event::Reset);
    }
}

mod undo_split {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        let event = timer.undo_split().unwrap();

        assert_eq!(event, Event::SplitUndone);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.undo_split().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn finished_run_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.undo_split().unwrap();

        assert_eq!(event, Event::SplitUndone);
    }

    #[test]
    fn cant_undo_first_split() {
        let mut timer = timer();

        timer.start().unwrap();
        let error = timer.undo_split().unwrap_err();

        assert_eq!(error, Error::CantUndoFirstSplit);
    }

    #[test]
    fn while_paused_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.pause().unwrap();
        let event = timer.undo_split().unwrap();

        assert_eq!(event, Event::SplitUndone);
    }
}

mod skip_split {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.skip_split().unwrap();

        assert_eq!(event, Event::SplitSkipped);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.skip_split().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn finished_run_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let error = timer.skip_split().unwrap_err();

        assert_eq!(error, Error::RunFinished);
    }

    #[test]
    fn cant_skip_last_split() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let error = timer.skip_split().unwrap_err();

        assert_eq!(error, Error::CantSkipLastSplit);
    }

    #[test]
    fn while_paused_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let event = timer.skip_split().unwrap();

        assert_eq!(event, Event::SplitSkipped);
    }
}

mod toggle_pause {
    use super::*;

    #[test]
    fn pause_works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.toggle_pause().unwrap();

        assert_eq!(event, Event::Paused);
    }

    #[test]
    fn resume_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let event = timer.toggle_pause().unwrap();

        assert_eq!(event, Event::Resumed);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.toggle_pause().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn finished_run_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let error = timer.toggle_pause().unwrap_err();

        assert_eq!(error, Error::RunFinished);
    }
}

mod toggle_pause_or_start {
    use super::*;

    #[test]
    fn start_works() {
        let mut timer = timer();

        let event = timer.toggle_pause_or_start().unwrap();

        assert_eq!(event, Event::Started);
    }

    #[test]
    fn pause_works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.toggle_pause_or_start().unwrap();

        assert_eq!(event, Event::Paused);
    }

    #[test]
    fn resume_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let event = timer.toggle_pause_or_start().unwrap();

        assert_eq!(event, Event::Resumed);
    }

    #[test]
    fn finished_run_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let error = timer.toggle_pause_or_start().unwrap_err();

        assert_eq!(error, Error::RunFinished);
    }
}

mod pause {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.pause().unwrap();

        assert_eq!(event, Event::Paused);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.pause().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn already_paused_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let error = timer.pause().unwrap_err();

        assert_eq!(error, Error::AlreadyPaused);
    }

    #[test]
    fn finished_run_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let error = timer.pause().unwrap_err();

        assert_eq!(error, Error::RunFinished);
    }
}

mod resume {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let event = timer.resume().unwrap();

        assert_eq!(event, Event::Resumed);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.resume().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn not_paused_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        let error = timer.resume().unwrap_err();

        assert_eq!(error, Error::NotPaused);
    }

    #[test]
    fn finished_run_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let error = timer.resume().unwrap_err();

        assert_eq!(error, Error::RunFinished);
    }
}

mod undo_all_pauses {
    use super::*;

    #[test]
    fn works_when_never_paused() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.undo_all_pauses().unwrap();

        assert_eq!(event, Event::PausesUndone);
    }

    #[test]
    fn works_when_paused() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        let event = timer.undo_all_pauses().unwrap();

        assert_eq!(event, Event::PausesUndoneAndResumed);
    }

    #[test]
    fn works_when_resumed() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause().unwrap();
        timer.resume().unwrap();
        let event = timer.undo_all_pauses().unwrap();

        assert_eq!(event, Event::PausesUndone);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.undo_all_pauses().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn finished_run_works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.undo_all_pauses().unwrap();

        assert_eq!(event, Event::PausesUndone);
    }
}

mod set_current_comparison {
    use super::*;

    #[test]
    fn setting_existing_comparison_works() {
        let mut timer = timer();

        let event = timer
            .set_current_comparison(comparison::personal_best::NAME)
            .unwrap();

        assert_eq!(event, Event::ComparisonChanged);
    }

    #[test]
    fn setting_non_existing_comparison_fails() {
        let mut timer = timer();

        let error = timer.set_current_comparison("non-existing").unwrap_err();

        assert_eq!(error, Error::ComparisonDoesntExist);
    }
}

mod toggle_timing_method {
    // Infallible
}

mod set_current_timing_method {
    // Infallible
}

mod initialize_game_time {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.initialize_game_time().unwrap();

        assert_eq!(event, Event::GameTimeInitialized);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.initialize_game_time().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn already_initialized_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.initialize_game_time().unwrap();
        let error = timer.initialize_game_time().unwrap_err();

        assert_eq!(error, Error::GameTimeAlreadyInitialized);
    }

    #[test]
    fn finished_run_works() {
        // FIXME: This behavior seems questionable.
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.initialize_game_time().unwrap();

        assert_eq!(event, Event::GameTimeInitialized);
    }
}

mod set_game_time {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.set_game_time(TimeSpan::default()).unwrap();

        assert_eq!(event, Event::GameTimeSet);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.set_game_time(TimeSpan::default()).unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn finished_run_works() {
        // FIXME: This behavior seems questionable.
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.set_game_time(TimeSpan::default()).unwrap();

        assert_eq!(event, Event::GameTimeSet);
    }
}

mod pause_game_time {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.pause_game_time().unwrap();

        assert_eq!(event, Event::GameTimePaused);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.pause_game_time().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn already_paused_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause_game_time().unwrap();
        let error = timer.pause_game_time().unwrap_err();

        assert_eq!(error, Error::GameTimeAlreadyPaused);
    }

    #[test]
    fn finished_run_works() {
        // FIXME: This behavior seems questionable.
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.pause_game_time().unwrap();

        assert_eq!(event, Event::GameTimePaused);
    }
}

mod resume_game_time {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        timer.pause_game_time().unwrap();
        let event = timer.resume_game_time().unwrap();

        assert_eq!(event, Event::GameTimeResumed);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.resume_game_time().unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn not_paused_fails() {
        let mut timer = timer();

        timer.start().unwrap();
        let error = timer.resume_game_time().unwrap_err();

        assert_eq!(error, Error::GameTimeNotPaused);
    }

    #[test]
    fn finished_run_works() {
        // FIXME: This behavior seems questionable.
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.pause_game_time().unwrap();
        let event = timer.resume_game_time().unwrap();

        assert_eq!(event, Event::GameTimeResumed);
    }
}

mod set_loading_times {
    use super::*;

    #[test]
    fn works() {
        let mut timer = timer();

        timer.start().unwrap();
        let event = timer.set_loading_times(TimeSpan::default()).unwrap();

        assert_eq!(event, Event::LoadingTimesSet);
    }

    #[test]
    fn without_a_run_fails() {
        let mut timer = timer();

        let error = timer.set_loading_times(TimeSpan::default()).unwrap_err();

        assert_eq!(error, Error::NoRunInProgress);
    }

    #[test]
    fn finished_run_works() {
        // FIXME: This behavior seems questionable.
        let mut timer = timer();

        timer.start().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        timer.split().unwrap();
        let event = timer.set_loading_times(TimeSpan::default()).unwrap();

        assert_eq!(event, Event::LoadingTimesSet);
    }
}

mod set_custom_variable {
    // Infallible
}
