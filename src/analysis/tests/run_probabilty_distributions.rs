use core::slice::Iter;
use crate::analysis::discontinuous_fourier_transforms::delta_function_fourier_series;
use super::super::discontinuous_fourier_transforms;

use std::f32::consts::TAU;
use std::iter::Map;

use rustfft::{FftPlanner, num_complex::{Complex, ComplexFloat}};

#[test]
fn example_test() {
    assert!("hello world".contains("hello"));
}

///
/// Makes sure that the discontinuous Fourier transform yeilds the correct values
///
#[test]
fn test_dirac_delta() {

    let duration = 10;

    // create the delta function, with a peak at t=0
    let mut delta_fourier = delta_function_fourier_series( TAU / duration as f32,
                                                      duration, 0.0);

    // create the FFT planner
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(duration);

    fft.process(&mut delta_fourier);

    // convert to real numbers by taking the magnitude of the complex numbers
    let magnitudes: Vec<f32> = delta_fourier.iter().map(|x: &Complex<f32>| -> f32 { (x.re().powi(2) + x.im().powi(2)).sqrt() }).collect();

    println!("{:?}", magnitudes);

}