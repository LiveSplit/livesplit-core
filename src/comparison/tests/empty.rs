use crate::comparison::{self, ComparisonGenerator};

fn test<T: ComparisonGenerator>(mut generator: T) {
    generator.generate(&mut [], &[]);
}

#[test]
fn average_segments() {
    test(comparison::AverageSegments);
}

#[test]
fn balanced_pb() {
    test(comparison::BalancedPB);
}

#[test]
fn best_segments() {
    test(comparison::BestSegments);
}

#[test]
fn best_split_times() {
    test(comparison::BestSplitTimes);
}

#[test]
fn latest_run() {
    test(comparison::LatestRun);
}

#[test]
fn median_segments() {
    test(comparison::MedianSegments);
}

#[test]
fn none() {
    test(comparison::None);
}

#[test]
fn worst_segments() {
    test(comparison::WorstSegments);
}
