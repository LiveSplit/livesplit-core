use std::f32::consts::TAU;

use assert_approx_eq::assert_approx_eq;
use rustfft::{FftPlanner, num_complex::{Complex, ComplexFloat}};

use core::slice::Iter;
use crate::analysis::statistical_pb_chance::discontinuous_fourier_transforms::{delta_function_dft, step_function_dft};

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

}

#[test]
fn test_unit_step() {
    let points = 16;

    let mut heaviside_fourier = step_function_dft(TAU / points as f32, points, 10.5f32);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(points);

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
