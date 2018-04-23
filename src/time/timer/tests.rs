use run::Editor;
use {Run, Segment, TimeSpan, Timer, TimingMethod};

fn run() -> Run {
    let mut run = Run::new();

    run.push_segment(Segment::new("A"));
    run.push_segment(Segment::new("B"));
    run.push_segment(Segment::new("C"));

    run
}

fn timer() -> Timer {
    let mut timer = Timer::new(run()).unwrap();

    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();

    timer
}

#[test]
fn monotically_increasing_split_times_after_resetting() {
    let mut timer = timer();

    let first = TimeSpan::from_seconds(5.0);
    timer.set_game_time(first);
    timer.split();

    let second = TimeSpan::from_seconds(15.0);
    timer.set_game_time(second);
    timer.split();

    let third = TimeSpan::from_seconds(10.0);
    timer.set_game_time(third);
    timer.split();

    let run = timer.into_run(true);

    // The first segment's time should be unchanged.
    assert_eq!(
        run.segment(0).personal_best_split_time().game_time,
        Some(first)
    );

    // 15.0 is larger than 5.0, so the second segment's time should be
    // unchanged.
    assert_eq!(
        run.segment(1).personal_best_split_time().game_time,
        Some(second)
    );

    // 10.0 is less than 15.0, and since we want split times to be monotonically
    // increasing, the third segment's time should be adjusted to 15.0.
    assert_eq!(
        run.segment(2).personal_best_split_time().game_time,
        Some(second)
    );
}

#[test]
fn deleting_best_segment_time_clears_segment_history() {
    let mut timer = timer();

    let first = TimeSpan::from_seconds(5.0);
    timer.set_game_time(first);
    timer.split();

    let second = TimeSpan::from_seconds(10.0);
    timer.set_game_time(second);
    timer.split();

    let third = TimeSpan::from_seconds(15.0);
    timer.set_game_time(third);
    timer.split();

    let run = timer.into_run(true);
    let run2 = run.clone();

    // =============================================

    let mut editor = Editor::new(run).unwrap();

    editor.select_timing_method(TimingMethod::GameTime);
    editor.select_only(1);
    editor.active_segment().set_split_time(None);

    // Clearing the second segment's split time should not affect the second
    // segment's best segment time.
    assert_eq!(
        editor.run().segment(1).best_segment_time().game_time,
        Some(second - first)
    );

    editor.active_segment().set_best_segment_time(None);

    // Since the second segment's split time is already cleared, clearing its
    // best segment time should clear the segment history and leave the best
    // segment time as blank.
    assert_eq!(
        editor.run().segment(1).segment_history().iter().next(),
        None
    );
    assert_eq!(editor.run().segment(1).best_segment_time().game_time, None);

    // =============================================

    editor = Editor::new(run2).unwrap();

    editor.select_timing_method(TimingMethod::GameTime);
    editor.select_only(1);
    editor.active_segment().set_best_segment_time(None);

    // Clearing the second segment's best segment time without clearing its
    // split time should still clear its segment history.
    assert_eq!(
        editor.run().segment(1).segment_history().iter().next(),
        None
    );
    // Since the second segment's split time was not cleared, the best segment
    // time should be fixed based on the personal best split times.
    assert_eq!(
        editor.run().segment(1).best_segment_time().game_time,
        Some(second - first)
    );
}

#[test]
fn modifying_best_segment_time_fixes_segment_history() {
    let mut timer = timer();

    let first = TimeSpan::from_seconds(5.0);
    timer.set_game_time(first);
    timer.split();

    let second = TimeSpan::from_seconds(10.0);
    timer.set_game_time(second);
    timer.split();

    let third = TimeSpan::from_seconds(15.0);
    timer.set_game_time(third);
    timer.split();

    let run = timer.into_run(true);
    let run2 = run.clone();

    // =============================================

    let mut editor = Editor::new(run).unwrap();

    editor.select_timing_method(TimingMethod::GameTime);
    editor.select_only(1);
    editor.active_segment().set_split_time(None);

    // Clearing the second segment's split time should not affect the second
    // segment's best segment time.
    assert_eq!(
        editor.run().segment(1).best_segment_time().game_time,
        Some(second - first)
    );

    let new_best = Some(TimeSpan::from_seconds(7.0));

    editor.active_segment().set_best_segment_time(new_best);

    // Changing the second segment's best segment time from 5.0 to 7.0 after
    // clearing the split time should change all times in the segment history so
    // that none are shorter than 7.0.
    assert_eq!(
        editor
            .run()
            .segment(1)
            .segment_history()
            .iter()
            .next()
            .and_then(|&(_, t)| t.game_time),
        new_best
    );
    // The second segment's best segment time should have changed to 7.0.
    assert_eq!(
        editor.run().segment(1).best_segment_time().game_time,
        new_best
    );

    // =============================================

    editor = Editor::new(run2).unwrap();

    editor.select_timing_method(TimingMethod::GameTime);
    editor.select_only(1);
    editor.active_segment().set_best_segment_time(new_best);

    // Changing the second segment's best segment time from 5.0 to 7.0 without
    // clearing the split time first should keep the segment history unaffected.
    // This is because the PB segment is still 5.0.
    assert_eq!(
        editor
            .run()
            .segment(1)
            .segment_history()
            .iter()
            .next()
            .and_then(|&(_, t)| t.game_time),
        Some(second - first)
    );
    // The second segment's best segment time should also be unaffected. This is
    // because the PB segment is still 5.0.
    assert_eq!(
        editor.run().segment(1).best_segment_time().game_time,
        Some(second - first)
    );
}

#[test]
fn import_pb_into_segment_history() {
    let mut editor = Editor::new(run()).unwrap();
    editor.select_timing_method(TimingMethod::GameTime);

    editor.select_only(0);
    let fake_first = Some(TimeSpan::from_seconds(5.0));
    editor.active_segment().set_split_time(fake_first);

    editor.select_only(1);
    let fake_second = Some(TimeSpan::from_seconds(10.0));
    editor.active_segment().set_split_time(fake_second);

    editor.select_only(2);
    let fake_third = Some(TimeSpan::from_seconds(15.0));
    editor.active_segment().set_split_time(fake_third);

    let run = editor.close();
    let mut timer = Timer::new(run).unwrap();

    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();

    let real_first = TimeSpan::from_seconds(4.0);
    timer.set_game_time(real_first);
    timer.split();

    let real_second = TimeSpan::from_seconds(9.0);
    timer.set_game_time(real_second);
    timer.split();

    let real_third = TimeSpan::from_seconds(13.0);
    timer.set_game_time(real_third);
    timer.split();

    timer.reset(true);

    let run = timer.run();

    // The previous PB's first segment should be imported into the segment
    // history with a non-positive index. A non-positive index means that it was
    // not done during an actual run.
    assert_eq!(
        run.segment(0)
            .segment_history()
            .get(-1)
            .and_then(|t| t.game_time),
        fake_first
    );
    // The new run's first segment should be imported into the segment history
    // with a positive index. A positive index means that it was done during an
    // actual run.
    assert_eq!(
        run.segment(0)
            .segment_history()
            .get(1)
            .and_then(|t| t.game_time),
        Some(real_first)
    );

    // The previous PB's second segment should not be imported into the segment
    // history. This is because it is a duplicate of the new run's second
    // segment.
    assert_eq!(run.segment(1).segment_history().get(-1), None);
    // The new run's second segment should be imported into the segment history
    // with a positive index.
    assert_eq!(
        run.segment(1)
            .segment_history()
            .get(1)
            .and_then(|t| t.game_time),
        Some(real_second - real_first)
    );

    // The previous PB's third segment should be imported into the segment
    // history with a non-positive index.
    assert_eq!(
        run.segment(2)
            .segment_history()
            .get(-1)
            .and_then(|t| t.game_time),
        catch! { fake_third? - fake_second? }
    );
    // The new run's third segment should be imported into the segment history
    // with a positive index.
    assert_eq!(
        run.segment(2)
            .segment_history()
            .get(1)
            .and_then(|t| t.game_time),
        Some(real_third - real_second)
    );
}

#[test]
fn import_pb_into_segment_history_and_remove_null_values() {
    let mut editor = Editor::new(run()).unwrap();
    editor.select_timing_method(TimingMethod::GameTime);

    editor.select_only(0);
    let fake_first = Some(TimeSpan::from_seconds(5.0));
    editor.active_segment().set_split_time(fake_first);

    editor.select_only(2);
    let fake_third = Some(TimeSpan::from_seconds(15.0));
    editor.active_segment().set_split_time(fake_third);

    let run = editor.close();
    let mut timer = Timer::new(run).unwrap();

    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();

    let real_first = TimeSpan::from_seconds(4.0);
    timer.set_game_time(real_first);
    timer.split();

    timer.skip_split();

    let real_third = TimeSpan::from_seconds(14.0);
    timer.set_game_time(real_third);
    timer.split();

    timer.reset(true);

    let run = timer.run();

    // The previous PB's first segment should be imported into the segment
    // history with a non-positive index. A non-positive index means that it was
    // not done during an actual run.
    assert_eq!(
        run.segment(0)
            .segment_history()
            .get(-1)
            .and_then(|t| t.game_time),
        fake_first
    );
    // The new run's first segment should be imported into the segment history
    // with a positive index. A positive index means that it was done during an
    // actual run.
    assert_eq!(
        run.segment(0)
            .segment_history()
            .get(1)
            .and_then(|t| t.game_time),
        Some(real_first)
    );

    // The previous PB's second segment should not be imported into the segment
    // history. The second segment's split time was blank for the previous PB,
    // so it is actually a part of a combined segment with the third segment.
    // Since the third segment is a duplicate of the new run's third segment,
    // neither the second nor the third segment should be imported.
    assert_eq!(run.segment(1).segment_history().get(-1), None);
    // The new run's second segment has a blank split time, so a null time
    // should be imported into the segment history with a positive index. This
    // indicates that it is a part of a combined segment with the third segment.
    assert_eq!(
        run.segment(1).segment_history().get(1).map(|t| t.game_time),
        Some(None)
    );

    // The previous PB's third segment should not be imported into the segment
    // history. This is because the third segment is a duplicate of the new
    // run's third segment.
    assert_eq!(run.segment(2).segment_history().get(-1), None);
    // The new run's third segment should be imported into the segment history
    // with a positive index.
    assert_eq!(
        run.segment(2)
            .segment_history()
            .get(1)
            .and_then(|t| t.game_time),
        Some(real_third - real_first)
    );
}

#[test]
fn import_best_segment_with_game_time_usage() {
    let mut timer = timer();

    let first = TimeSpan::from_seconds(5.0);
    timer.set_game_time(first);
    timer.split();

    let run = timer.into_run(true);
    let mut editor = Editor::new(run).unwrap();

    editor.select_timing_method(TimingMethod::GameTime);

    editor.select_only(0);
    let best = Some(TimeSpan::from_seconds(4.0));
    editor.active_segment().set_best_segment_time(best);

    editor.insert_segment_above();

    let history = editor.run().segment(0).segment_history();
    // The newly inserted segment's history should have a null time with a
    // non-positive index. This represents a skipped split for the imported best
    // segment with a time of 4.0.
    assert_eq!(history.get(0).map(|t| t.game_time), Some(None));
    // The newly inserted segment's history should have a null time with a
    // positive index. This represents a skipped split for an actual run with a
    // time of 5.0.
    assert_eq!(history.get(1).map(|t| t.game_time), Some(None));

    let history = editor.run().segment(1).segment_history();
    // The second segment's history should have a time of 4.0 with a
    // non-positive index, which represents the imported best segment.
    assert_eq!(history.get(0).and_then(|t| t.game_time), best);
    // The second segment's history should have a time of 5.0 with a positive
    // index, which represents an actual run.
    assert_eq!(history.get(1).and_then(|t| t.game_time), Some(first));
}

#[test]
fn clears_run_id_when_pbing() {
    let mut timer = timer();

    // Get a PB

    timer.set_current_timing_method(TimingMethod::GameTime);

    let first = TimeSpan::from_seconds(5.0);
    timer.set_game_time(first);
    timer.split();

    let second = TimeSpan::from_seconds(10.0);
    timer.set_game_time(second);
    timer.split();

    let third = TimeSpan::from_seconds(15.0);
    timer.set_game_time(third);
    timer.split();

    let mut run = timer.into_run(true);

    // Set the run id

    assert_eq!(run.metadata().run_id(), "");
    run.metadata_mut().set_run_id("34567");
    assert_eq!(run.metadata().run_id(), "34567");

    // Do a new run, but this time don't pb. Run ID should be the same.

    let mut timer = Timer::new(run).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);

    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();

    let first = TimeSpan::from_seconds(6.0);
    timer.set_game_time(first);
    timer.split();

    let second = TimeSpan::from_seconds(11.0);
    timer.set_game_time(second);
    timer.split();

    let third = TimeSpan::from_seconds(16.0);
    timer.set_game_time(third);
    timer.split();

    timer.reset(true);

    assert_eq!(timer.run().metadata().run_id(), "34567");

    // Do a new run and PB. Run ID should be cleared.

    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();

    let first = TimeSpan::from_seconds(4.0);
    timer.set_game_time(first);
    timer.split();

    let second = TimeSpan::from_seconds(9.0);
    timer.set_game_time(second);
    timer.split();

    let third = TimeSpan::from_seconds(14.0);
    timer.set_game_time(third);
    timer.split();

    timer.reset(true);

    assert_eq!(timer.run().metadata().run_id(), "");
}
