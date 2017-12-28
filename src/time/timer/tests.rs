use {Run, Segment, TimeSpan, Timer, TimingMethod};
use run::Editor;

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

    assert_eq!(
        run.segment(0).personal_best_split_time().game_time,
        Some(first)
    );

    assert_eq!(
        run.segment(1).personal_best_split_time().game_time,
        Some(second)
    );

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

    assert_eq!(
        editor.run().segment(1).best_segment_time().game_time,
        Some(second - first)
    );

    editor.active_segment().set_best_segment_time(None);

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

    assert_eq!(
        editor.run().segment(1).segment_history().iter().next(),
        None
    );
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

    assert_eq!(
        editor.run().segment(1).best_segment_time().game_time,
        Some(second - first)
    );

    let new_best = Some(TimeSpan::from_seconds(7.0));

    editor.active_segment().set_best_segment_time(new_best);

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
    assert_eq!(
        editor.run().segment(1).best_segment_time().game_time,
        new_best
    );

    // =============================================

    editor = Editor::new(run2).unwrap();

    editor.select_timing_method(TimingMethod::GameTime);
    editor.select_only(1);
    editor.active_segment().set_best_segment_time(new_best);

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

    assert_eq!(
        run.segment(0)
            .segment_history()
            .get(-1)
            .and_then(|t| t.game_time),
        fake_first
    );
    assert_eq!(
        run.segment(0)
            .segment_history()
            .get(1)
            .and_then(|t| t.game_time),
        Some(real_first)
    );

    assert_eq!(run.segment(1).segment_history().get(-1), None);
    assert_eq!(
        run.segment(1)
            .segment_history()
            .get(1)
            .and_then(|t| t.game_time),
        Some(real_second - real_first)
    );

    assert_eq!(
        run.segment(2)
            .segment_history()
            .get(-1)
            .and_then(|t| t.game_time),
        catch! { fake_third? - fake_second? }
    );
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

    assert_eq!(
        run.segment(0)
            .segment_history()
            .get(-1)
            .and_then(|t| t.game_time),
        fake_first
    );
    assert_eq!(
        run.segment(0)
            .segment_history()
            .get(1)
            .and_then(|t| t.game_time),
        Some(real_first)
    );

    assert_eq!(run.segment(1).segment_history().get(-1), None);
    assert_eq!(
        run.segment(1).segment_history().get(1).map(|t| t.game_time),
        Some(None)
    );

    assert_eq!(run.segment(2).segment_history().get(-1), None);
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
    assert_eq!(history.get(0).map(|t| t.game_time), Some(None));
    assert_eq!(history.get(1).map(|t| t.game_time), Some(None));

    let history = editor.run().segment(1).segment_history();
    assert_eq!(history.get(0).and_then(|t| t.game_time), best);
    assert_eq!(history.get(1).and_then(|t| t.game_time), Some(first));
}
