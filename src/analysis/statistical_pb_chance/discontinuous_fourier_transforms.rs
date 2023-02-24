use rustfft::{FftPlanner, num_complex::Complex};
use std::f64::consts::TAU;

///
/// Functions that calculate the Fourier transforms of crucial discontinuous functions (Dirac Delta
/// and unit step functions)
///

///
/// computes the discrete fourier transform of a dirac delta function
///
/// # Arguments
///
/// * `omega_naught` - The fundamental frequency of the fourier transform we are computing
/// * `num_terms` - The number of terms in the fourier transform to return
/// * `time` - The point in time at which the delta function is nonzero
///
/// # Mathematics
///
/// The fourier transform of a delta function is given by:
///
/// ```latex
/// X[k] = e^{-t_{0}ωi}
/// ```
///
/// Where k is the index of the discrete fourier transform, t0 is the time at which the delta function
/// is nonzero, ω is the fundamental frequency (ω = k * ω0) and i is the complex unit (√(-1))
///
///
pub fn delta_function_dft(omega_naught: f32, num_terms: usize, time: f32) -> Vec<Complex<f32>> {

    // determine the halfway point of the array. This is important because the frequencies in the second
    // half of the transform are effectively negative frequencies. Since the formula for the fourier transform
    // uses the frequency outside of a complex exponential we need to use the negative
    // index of the second half of the array.
    let cutoff = (num_terms as f32 / 2.0).ceil() as usize;

    // initialize the fourier series
    let mut fourier_series= vec![Complex::<f32>{re: 0.0, im: 0.0}; num_terms];

    for k in 0..cutoff {
        let omega = omega_naught * k as f32;
        // println!("Omega: {}", omega);
        fourier_series[k] = Complex::<f32>{re: 0.0, im: - omega * time}.exp();
    }

    for k in cutoff..num_terms {
        let omega = omega_naught * (k as i64 - num_terms as i64) as f32; // different calculation of omega
        // println!("Omega: {}", omega);
        fourier_series[k] = Complex::<f32>{re: 0.0, im: - omega * time}.exp();
    }

    return fourier_series;
}

///
/// computes the discrete fourier transform of a unit step function. In particular, this function
/// returns a step down at the specified point
///
/// # Arguments
///
/// * `omega_naught` - the fundamental frequency of the fourier transform we are computing
/// * `num_terms` - the number of terms in the fourier transform to return
/// * `time` - the point in time at which the unit step function drops
///
/// # Mathematics
///
/// The fourier transform of a unit step function is given by:
///
/// ```latex
/// x[t] = u[t-t0] <=> X[k] = (𝛅[k] + 1 / (iω)) * e^{-iωt_{0}}
/// ```
///
/// Where k is the index of the discrete fourier transform, t0 is the point in time at which the
/// unit step function goes high, ω is the fundamental frequency (ω = k * ω0) and i is the complex unit (√(-1))
///
/// Unlike the dirac delta function, when we consider the unit step function, we must note that due to
/// the cyclic nature of the time domain, our step function must contain both a step up and a step down.
/// The definition of the unit step function seen above contains only a step up. We must therefore
/// add two step functions using the above definitions to obtain a true step function
///
/// The unit step function returned by this function is initially 1, then steps down to zero at the point `time`
/// The step function must therefore step up between the last element of the array and the first element. i.e.
/// the index -0.5. For this reason. Thus, the fourier transform returned by this function yields:
///
/// ```latex
/// u[t+0.5] - u[t-t_{0}]
/// ```
///
/// From the linearity of the Fourier transform and the distributive property, we get
///
/// ```latex
/// u[t+0.5] - u[t-t_{0}] <=> X[k] = (𝛅[k] + 1 / (iω)) * e^{-iω(-1/2)} - (𝛅[k] + 1 / (iω)) * e^{-iωt_{0}}
/// X[k] = (𝛅[k] + 1 / (iω)) * (e^{-iω(-1/2)} - e^{-iωt_{0}})
/// ```
///
pub fn step_function_dft(omega_naught: f32, num_terms: usize, time: f32) -> Vec<Complex<f32>> {
    let mut fourier_series = vec![Complex::<f32>{re: 0.0, im: 0.0}; num_terms];

    // determine the halfway point of the array. This is important because the frequencies in the second
    // half of the transform are effectively negative frequencies. Since the formula for the fourier transform
    // uses the frequency outside of a complex exponential (in particular, as 1/ω) we need to use the negative
    // index of the second half of the array.
    let cutoff = (num_terms as f32 / 2.0).ceil() as usize;

    // 1/ω is undefined for ω=0, so we need to manually specify it
    // X[0] is equal to the integral of the whole function. In our case, that's a unit step function
    // that starts at -0.5 and ends at `time`. Therefore, the integral is time - (-0.5) = time + 0.5
    fourier_series[0] = Complex::<f32>{re: (time + 0.5), im: 0.0};

    for k in 1..cutoff {
        let omega = omega_naught * k as f32;
        // println!("Omega: {}", omega);
        fourier_series[k] = Complex::<f32>{re: 0.0, im: - 1.0 / omega} * // 1/jω
            (Complex::<f32>{re: 0.0, im: - omega * (-0.5)}.exp() - // e^{-iω(-1/2)}
             Complex::<f32>{re: 0.0, im: - omega * (time)}.exp()); // e^{-iωt_{0}}
    }

    for k in cutoff..num_terms {
        let omega = omega_naught * (k as i64 - num_terms as i64) as f32; // different calculation of omega
        // println!("Omega: {}", omega);
        fourier_series[k] = Complex::<f32>{re: 0.0, im: - 1.0 / omega} * // 1/jω
            (Complex::<f32>{re: 0.0, im: - omega * (-0.5)}.exp() - // e^{-iω(-1/2)}
                Complex::<f32>{re: 0.0, im: - omega * (time)}.exp()); // e^{-iωt_{0}}
    }

    return fourier_series;
}