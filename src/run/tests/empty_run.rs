use crate::Run;

#[test]
fn min_segment_history_index() {
    assert!(Run::new().min_segment_history_index().is_none());
}

#[test]
fn start_next_run() {
    Run::new().start_next_run();
}

#[test]
fn max_attempt_history_index() {
    assert!(Run::new().max_attempt_history_index().is_none());
}

#[test]
fn fix_splits() {
    Run::new().fix_splits();
}

#[test]
fn clear_history() {
    Run::new().clear_history();
}

#[test]
fn clear_times() {
    Run::new().clear_times();
}

#[test]
fn import_pb_into_segment_history() {
    Run::new().import_pb_into_segment_history();
}

#[test]
#[should_panic]
fn import_best_segment() {
    Run::new().import_best_segment(0);
}

#[test]
#[should_panic(expected = "There is no attempt in the Attempt History.")]
fn update_segment_history() {
    Run::new().update_segment_history(0);
}
