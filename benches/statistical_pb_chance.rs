use criterion::{criterion_group, criterion_main, Criterion, black_box};
use livesplit_core::{RealTime, SegmentHistory, Time, TimeSpan, TimingMethod};
use livesplit_core::analysis::statistical_pb_chance::probability_distribution::ProbabilityDistribution;


criterion_main!(benches);
criterion_group!(benches, compute_probability_8192, add_distributions);

///
/// Computes a CDF of a probability distribution with 8192 data points
///
fn compute_probability_8192(c: &mut Criterion){

    // initialize the distribution
    let mut times = SegmentHistory::default();

    times.insert(1, Time::from(RealTime(Some(TimeSpan::from_seconds(1.2)))));
    times.insert(2, Time::from(RealTime(Some(TimeSpan::from_seconds(6.0)))));

    let dist = ProbabilityDistribution::new(&times, TimingMethod::RealTime,
                                            10.0, 8192, 0.5);

    c.bench_function("Probability Less than x (8192 points)", move |b| {
        b.iter(|| dist.probability_below(black_box(5.0)))
    });

}

///
/// benchmarks adding two distributions together
///
fn add_distributions(c: &mut Criterion){
    // initialize the distribution
    let mut times = SegmentHistory::default();

    times.insert(1, Time::from(RealTime(Some(TimeSpan::from_seconds(1.2)))));
    times.insert(2, Time::from(RealTime(Some(TimeSpan::from_seconds(6.0)))));

    let dist = ProbabilityDistribution::new(&times, TimingMethod::RealTime,
                                            10.0, 8192, 0.5);

    let other = dist.clone();

    c.bench_function("Adding Distributions", |b| {
        b.iter(|| &dist + &other)
    });
}