use std::f32::consts::TAU;
use std::ops;
use std::sync::Arc;
use rustfft::{FftPlanner, Fft};
use rustfft::num_complex::{Complex, ComplexFloat};

use crate::SegmentHistory;
use crate::timing::TimingMethod;
use crate::analysis::statistical_pb_chance::discontinuous_fourier_transforms::{delta_function_dft, step_function_dft};

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
/// For more details, see [Wikipedia](https://en.wikipedia.org//wiki/Probability_density_function), 3Blue1Brown also has a [video](https://www.3blue1brown.com/lessons/pdfs) on the subject,
/// but the central example of this video is irreverent to what is done here
///
/// # Usage
///
///
///
/// # Internal Functionality
///
/// There are two computationally expensive tasks necessary to use probability distributions to compute
///
///

pub struct ProbabilityDistribution {

    max_duration: f32, // the maximum simulated time duration
    omega_naught: f32, // the fundamental frequency of the fourier transform of the distribution

    transform: Vec<Complex<f32>>, // Fourier coefficients

    fft_inverse: Arc<dyn Fft<f32>>

}

impl ProbabilityDistribution {

    ///
    /// Initializes a probability distribution given a segment history weighted by an exponentially weighted
    /// average with smoothing factor `smoothing_factor`
    ///
    /// # Arguments
    ///
    /// * `times` - SegmentHistory object containing the history of times on the specified split
    /// * `method` - The timing method to use to create the distribution
    /// * `max_duration` - the maximum* duration of the run
    /// * `num_terms` - the number of points to record data for in the distribution
    /// * `smoothing_factor` - Smoothing factor of the exponential smoothing applied to the SegmentHistory (See [here](https://en.wikipedia.org/wiki/Exponential_smoothing))
    ///
    /// *The time of the run can be larger than this maximum duration under most circumstances. Problems arise
    /// when the duration of the run is close to the maximum duration of the run.
    ///
    pub fn new(times: &SegmentHistory,
               method: TimingMethod,
               max_duration: f32, num_terms: usize, smoothing_factor: f32) -> Self {

        // create a planner
        let mut planner: FftPlanner<f32> = FftPlanner::new();

        // initialize the distribution
        let mut dist = ProbabilityDistribution {
            max_duration,
            omega_naught: TAU / max_duration,
            transform: vec![Complex::<f32>{re: 0.0, im: 0.0}; num_terms],
            fft_inverse: planner.plan_fft_inverse(num_terms)
        };

        // println!("omega naught: {}", dist.omega_naught);
        //
        // println!("{:?}", dist.transform);

        // go through all the splits and add them to the distribution
        for (history_index, split) in times.iter_actual_runs().enumerate() {

            let time = split.1[method].expect(&*format!("Split does not include the specified timing method {}", if method == TimingMethod::RealTime {"Real Time"} else {"In-Game Time"})).total_seconds() as f32;
            let dft = delta_function_dft(dist.omega_naught, num_terms, time);

            // println!("{:?}", dft);

            // on the very first segment, there is no weighing, so we just insert the time with no weight
            if history_index == 0 {
                dist.transform = dft;
            }
            else {
                // add the two vectors element wise in the form of an exponentially weighted average
                for frequency_index in 0..num_terms {
                    dist.transform[frequency_index] = dft[frequency_index] * smoothing_factor + (1.0 - smoothing_factor) * dist.transform[frequency_index];
                }
            }
        }

        // println!("after adding the points");
        // println!("{:?}", dist.transform);

        return dist;

    }

    ///
    /// Computes the Probability of the distribution being less than the specified amount
    ///
    /// # Arguments
    ///
    /// * `time` - The point in time below which we wish to know the probability of being
    ///
    pub fn probability_below(&self, time: f32) -> f32{

        // to take the integral, we evaluate the fourier transform at omega = 0. But this gives us the integral from -inf to inf.
        // we want to limit our integral, which we can do by multiplying by a unit step function. In the frequency domain,
        // this corresponds to taking a convolution. Since we only need to evaluate the result of the convolution at one point, all we're
        // really doing is a multiply-add computation

        let step = step_function_dft(self.omega_naught, self.transform.len(), time);

        // to compute the convolution, we multiply the distribution and the step function element wise
        // with one in reverse order and add them together. For a given index `i`, the correct product is
        // calculated by multiplying element `i` of one function by `N - i` in the other function, where N
        // is the number of elements in the arrays. For the case of i = 0 however, this results in us evaluating
        // the array at index N, which is out of bounds. For this reason, we skip the first element and add
        // it's product at the very end.
        let convolution = (1..step.len()).map(|index| -> Complex<f32> {
            // println!("{}", self.transform[index] * step[step.len() - index]);
            return self.transform[index] * step[step.len() - index];
        });

        let integral = (convolution.sum::<Complex<f32>>() + self.transform[0] * step[0]).abs();

        return f32::max(f32::min(integral / self.max_duration as f32, 1.0), 0.0);
    }

}

impl ops::Add<ProbabilityDistribution> for ProbabilityDistribution {
    type Output = ProbabilityDistribution;

    fn add(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        let mut result: ProbabilityDistribution = ProbabilityDistribution {
            max_duration: self.max_duration,
            omega_naught: self.omega_naught,
            transform: Vec::with_capacity(self.transform.capacity()),
            fft_inverse: self.fft_inverse
        };

        // multiply the elements
        for i in 0..self.transform.len() {
            result.transform[i] = self.transform[i] * other.transform[i];
        }

        return result;
    }
}

impl ops::AddAssign<ProbabilityDistribution> for ProbabilityDistribution {

    fn add_assign(&mut self, rhs: ProbabilityDistribution){
        for i in 0..self.transform.len() {
            self.transform[i] *= rhs.transform[i];
        }
    }
}