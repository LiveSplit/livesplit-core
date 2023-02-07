use std::ops;
use rustfft::num_complex::Complex;
use crate::{Segment, TimeSpan, TimingMethod};

/// Utilities for handling Probability Distributions
///
/// # Overview of Probability Distributions
///
/// "Probability Distributions", or "Probability Density Functions" are essentially continuous histograms. The describe the relationship
/// between possible times and the probability of obtaining them. The odds that the random variable
/// will lie between points A and B is the integral from A to B of the probability density function.
/// The "Skill curve" used elsewhere is essentially the integral of a probability distribution.
/// Both methods contain the same information, however the math required to combine probability distributions
/// can be optimized better than a skill curve can be.
///
/// #
///
/// The
///
/// # Internal Functionality
///
/// There are two computationally expensive tasks necessary to use probability distributions to compute
///
///

struct ProbabilityDistribution {

    max_duration: f32, // the maximum simulated time duration
    omega_naught: f32, // the fundamental frequency of the fourier transform of the distribution

    transform: Vec<Complex<f32>> // Fourier coefficients



}

impl ProbabilityDistribution {

    // pub fn new() -> Self {
    //
    // }

    // pub fn probability_below(x: f32) -> f32{
    //
    // }

}

impl ops::Add<ProbabilityDistribution> for ProbabilityDistribution {
    type Output = ProbabilityDistribution;

    fn add(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        let mut result: ProbabilityDistribution = ProbabilityDistribution {
            max_duration: self.max_duration,
            omega_naught: self.omega_naught,
            transform: Vec::with_capacity(self.transform.capacity()),
        };

        // multiply the elements
        for i in 0..self.transform.len() {
            result.transform[i] = self.transform[i] * other.transform[i];
        }

        return result;
    }
}