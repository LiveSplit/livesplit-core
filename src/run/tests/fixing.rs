use crate::tests_helper::{create_timer, run_with_splits};

#[test]
fn reattaches_unattached_segment_history_elements_by_using_negative_ids() {
    let mut timer = create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[3.0, 6.0]);
    run_with_splits(&mut timer, &[2.0, 4.0]);
    let mut run = timer.into_run(true);

    // We pop the last attempt from the history, but keep it in the segment
    // history, which makes the segment history elements unattached.
    run.attempt_history.pop().unwrap();

    run.fix_splits();

    let segments = run.segments();

    assert_eq!(segments[0].segment_history().try_get_min_index(), Some(0));
    assert_eq!(segments[0].segment_history().try_get_max_index(), Some(1));

    assert_eq!(segments[1].segment_history().try_get_min_index(), Some(0));
    assert_eq!(segments[1].segment_history().try_get_max_index(), Some(1));
}
