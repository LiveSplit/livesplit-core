use std::f32::consts::TAU;

use assert_approx_eq::assert_approx_eq;
use rustfft::{Fft, FftPlanner, num_complex::{Complex, ComplexFloat}};

use core::slice::Iter;
use std::any::Any;
use std::sync::Arc;
use time::Duration;
use crate::analysis::statistical_pb_chance::discontinuous_fourier_transforms::{delta_function_dft, step_function_dft};
use crate::analysis::statistical_pb_chance::probability_distribution::{ProbabilityDistribution};
use crate::{RealTime, SegmentHistory, Time, TimeSpan, TimingMethod};


#[test]
fn example_test() {
    assert!("hello world".contains("hello"));
}

///
/// Makes sure that the discontinuous Fourier transform yields the correct values
///
#[test]
fn test_ten_element_dirac_delta() {

    let duration = 10;

    // create the delta function, with a peak at t=0
    let mut delta_fourier = delta_function_dft( TAU / duration as f32,
                                                      duration, 0.0);

    // create the FFT planner
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(duration);

    fft.process(&mut delta_fourier);

    // convert to real numbers by taking the magnitude of the complex numbers
    let magnitudes: Vec<f32> = delta_fourier.iter().map(|x: &Complex<f32>| -> f32 { (x.re().powi(2) + x.im().powi(2)).sqrt() / delta_fourier.len() as f32}).collect();

    assert_approx_eq!(magnitudes[0], 1.0);

    for i in 1..magnitudes.len() {
        assert_approx_eq!(magnitudes[i], 0.0);
    }

}

///
/// Makes sure that the fourier transform yields the correct result when the duration does not equal
/// the number of points
///
#[test]
fn test_dirac_delta_different_duration() {

    let points = 16;

    // create the delta function, with a peak at t=0
    let mut delta_fourier = delta_function_dft( TAU / 10 as f32,
                                                points, 0.0);

    // create the FFT planner
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(points);

    fft.process(&mut delta_fourier);

    // convert to real numbers by taking the magnitude of the complex numbers
    let magnitudes: Vec<f32> = delta_fourier.iter().map(|x: &Complex<f32>| -> f32 { x.abs() / delta_fourier.len() as f32}).collect();

    assert_approx_eq!(magnitudes[0], 1.0);

    for i in 1..magnitudes.len() {
        assert_approx_eq!(magnitudes[i], 0.0);
    }

}///
/// Makes sure that the fourier transform yields the correct result when the duration does not equal
/// the number of points
///
#[test]
fn test_dirac_delta_non_integer() {

    let points = 16;

    // create the delta function, with a peak at t=0
    // [Complex { re: 1.0, im: -0.0 }, Complex { re: 0.72896856, im: -0.6845472 }, Complex { re: 0.06279038, im: -0.9980267 }, Complex { re: -0.6374242, im: -0.7705131 }, Complex { re: -0.9921147, im: -0.12533297 }, Complex { re: -0.8090168, im: 0.58778554 }, Complex { re: -0.18738091, im: 0.98228735 }, Complex { re: 0.535827, im: 0.8443278 }, Complex { re: 0.9685833, im: 0.24868935 }, Complex { re: 0.87630624, im: -0.48175442 }, Complex { re: 0.30901635, im: -0.9510567 }, Complex { re: -0.42577976, im: -0.9048268 }, Complex { re: -0.9297768, im: -0.3681238 }, Complex { re: -0.9297761, im: 0.36812562 }, Complex { re: -0.42577887, im: 0.90482724 }, Complex { re: 0.30901775, im: 0.95105624 }]
    let mut delta_fourier = delta_function_dft( TAU / 10f32,
                                                points, 1.2);

    println!("{:?}", delta_fourier);

    // create the FFT planner
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(points);

    fft.process(&mut delta_fourier);

    // convert to real numbers by taking the magnitude of the complex numbers
    let magnitudes: Vec<f32> = delta_fourier.iter().map(|x: &Complex<f32>| -> f32 { x.abs() / delta_fourier.len() as f32}).collect();

    println!("{:?}", magnitudes);

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[test]
fn test_unit_step() {
    let points = 16;

    let mut heaviside_fourier = step_function_dft(TAU / points as f32, points, 10.5f32);

    let mut planner = FftPlanner::new();
    let fft: Arc<dyn Fft<f32>> = planner.plan_fft_inverse(points);


    fft.process(&mut *heaviside_fourier);

    // convert to real numbers by taking the magnitude of the complex numbers
    let magnitudes: Vec<f32> = heaviside_fourier.iter().map(|x: &Complex<f32>| -> f32 { x.abs() / points as f32}).collect();

    for i in 0..11 {
        assert_approx_eq!(magnitudes[i], 1f32, 0.1); // high tolerance b/c this isn't a precise unit step function
    }

    for i in 11..points {
        assert_approx_eq!(magnitudes[i], 0f32, 0.1);
    }

}

#[test]
fn test_init_probability_distribution() {

    let mut times = SegmentHistory::default();

    times.insert(1, Time::from(RealTime(Some(TimeSpan::from_seconds(1.2)))));
    // times.insert(2, Time::from(RealTime(Some(TimeSpan::from_seconds(2.0)))));
    // times.insert(3, Time::from(RealTime(Some(TimeSpan::from_seconds(4.5)))));
    // times.insert(4, Time::from(RealTime(Some(TimeSpan::from_seconds(5.3)))));
    // times.insert(5, Time::from(RealTime(Some(TimeSpan::from_seconds(5.5)))));

    let dist = ProbabilityDistribution::new(&times, TimingMethod::RealTime,
                                            10.0, 16, 1.0);

    let limit = 5.0;

    println!("There is a {}% chance of being less than, {}", dist.probability_below(limit) * 100.0, limit);

    // for time in times.iter() {
    //
    //     println!("{}", time.1.real_time.expect("time empty").total_seconds());
    //
    // }
}