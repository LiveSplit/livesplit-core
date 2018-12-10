use super::super::sum_of_segments::{calculate_best, calculate_worst};
use super::super::total_playtime;
use crate::{Run, TimeSpan, TimingMethod};

#[test]
fn sum_of_best() {
    let run = Run::new();
    assert_eq!(
        calculate_best(run.segments(), false, false, TimingMethod::RealTime),
        Some(TimeSpan::zero())
    );
    assert_eq!(
        calculate_best(run.segments(), false, true, TimingMethod::RealTime),
        Some(TimeSpan::zero())
    );
    assert_eq!(
        calculate_best(run.segments(), true, false, TimingMethod::RealTime),
        Some(TimeSpan::zero())
    );
    assert_eq!(
        calculate_best(run.segments(), true, true, TimingMethod::RealTime),
        Some(TimeSpan::zero())
    );
}

#[test]
fn sum_of_worst() {
    let run = Run::new();
    assert_eq!(
        calculate_worst(run.segments(), false, TimingMethod::RealTime),
        Some(TimeSpan::zero())
    );
    assert_eq!(
        calculate_worst(run.segments(), true, TimingMethod::RealTime),
        Some(TimeSpan::zero())
    );
}

#[test]
fn total_playtime() {
    let run = Run::new();
    assert_eq!(total_playtime::calculate(&run), TimeSpan::zero());
}
