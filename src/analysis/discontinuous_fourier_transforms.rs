use rustfft::{FftPlanner, num_complex::Complex};
use std::f64::consts::TAU;

///
/// Functions that calculate the Fourier transforms of crucial discontinuous functions (Dirac Delta
/// and unit step functions)
///

///
/// computes the fourier
///
/// # Arguments
///
/// * `omega_naught` - the fundamental frequency of the fourier transform we are computing
/// * `num_terms` - the number of terms in the fourier transform to return
/// * `time` - the point in time at which the delta function is nonzero
///
///
///
pub fn delta_function_fourier_series(omega_naught: f32, num_terms: usize, time: f32) -> Vec<Complex<f32>> {

    // initialize the fourier series
    let mut fourier_series= vec![Complex::<f32>{re: 0.0, im: 0.0}; num_terms];

    let cutoff = (num_terms as f32 / 2.0 + 1.0).ceil() as usize;

    // insert the positive elements into the array
    for n in 0..cutoff {
        let temp: Complex<f32> = Complex::new(0.0, - (n as f32) * omega_naught * time);
        fourier_series[n] = temp.exp();
    }

    // insert the negative elements
    for n in cutoff..num_terms {
        fourier_series[n] = Complex::new(0.0, - (n as f32 - num_terms as f32) * omega_naught * time).exp();
    }

    return fourier_series;
}