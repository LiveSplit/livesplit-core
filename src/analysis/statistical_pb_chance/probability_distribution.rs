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
/// In the case of speedrunning, the Random variable is the time of a split or the time of the full run.
/// Therefore, the probability of a speedrunner getting a time between A and B would be the integral
/// from A to B of the probability density function.
///
/// Probability Distributions
///
/// The "Skill curve" used elsewhere is essentially the integral of a probability distribution.
/// Both methods contain the same information, however the math required to combine probability distributions
/// can be optimized better than a skill curve can be.
///
/// For more details, see [Wikipedia](https://en.wikipedia.org//wiki/Probability_density_function),
/// 3Blue1Brown also has a [video](https://www.3blue1brown.com/lessons/pdfs) on the subject,
/// but the central example of this video is irreverent to what is done here
///
/// # Usage
///
/// A probability distribution can be created from a split history object
///
/// ```ignore
/// use livesplit_core::{RealTime, SegmentHistory, TimingMethod};
/// use livesplit_core::analysis::statistical_pb_chance::probability_distribution::ProbabilityDistribution;
///
/// // initialize some segment history
/// let history: SegmentHistory; // ...
///
/// // Max Duration: 2 Hrs = 7200 seconds. Amost always still works the run lasts more than Max_Duration.
/// // Calculations will be inaccurate only if the total run time is close to a multiple of the Max Duration (2 hours, 4 hours, 6 hours, etc.)
/// // num_terms: Number of terms in Fourier Transform (fewer terms = faster, more terms = greater precision)
/// // smoothing_factor: The most recent split makes up 20% of the distribution (20% = 0.2)
/// let dist = ProbabilityDistribution::new(&history, method: TimingMethod::RealTime, max_duration: 7200, num_terms: 8192, smoothing_factor: 0.2);
/// ```
/// (See `ProbabilityDistribution::new` for more details)
///
/// Once a probability Distribution has been created, additional splits can be added to it.
///
/// ```ignore
/// // adding a time of 2:12.7 to the split with weight 0.2
/// dist.add_point(2 * 60 + 12.7, 0.2)
/// ```
///
///
/// There are two main pieces of functionality associated with probability distributions
///
/// 1) Combining the distributions of all splits in the run to generate the distribution of possible final
/// times for the run
/// 2) Using the distribution of the possible final times to determine the probability that the final
/// time will be below a desired goal or objective.
///
/// The distributions of two splits can be added to each other by simply using the overloaded `add` trait
///
/// ```ignore
/// use livesplit_core::analysis::statistical_pb_chance::probability_distribution::ProbabilityDistribution;
///
/// // create from segment history
/// let cap_kingdom: ProbabilityDistribution;
/// let cascade_kingdom: ProbabilityDistribution;
/// let sand_kingdom: ProbabilityDistribution;
/// let lake_kingdom: ProbabilityDistribution;
/// // ...
///
/// // compute the distributions of the remaining kingdoms
/// let full_run = cap_kingdom + cascade_kingdom + sand_kingdom + lake_kingdom; // + ...
/// let after_cap = cascade_kingdom + sand_kingdom + lake_kingdom; // + ...
/// let after_cascade = sand_kingdom + lake_kingdom; // + ...
/// // ...
///
/// ```
///
/// To determine the probability that a given distribution is below some specific amount, simply use
/// the `probability_below` method
///
/// ```ignore
/// let sub_hour = full_run.probability_below(60 * 60);
/// let below_55_minutes_after_cap = after_cap.probability_below(55 * 60);
/// ```
///
///
/// # Internal Functionality
///
/// Instead of tracking the data for the full probability distribution, the [Fourier Transform](https://en.wikipedia.org/wiki/Fourier_transform)
/// of the distribution is tracked to allow for extremely efficient computations.
///
/// The PDF of the sum of two random variables is equal to the convolution of the PDFs of the two random
/// variables being added. The convolution theorem states that convolution in the time domain corresponds to
/// multiplication in the frequency domain. Therefore, the distribution of the sum of any two splits is computed
/// by complex multiplication.
///
/// The probability that a random variable will be less than a given point is a bit more difficult to explain.
/// We wish to calculate...
///
/// $$\int_{-\infty}^{t}p(x)dx$$
///
/// ...where p(x) is the probability density function. We observe that the integral is very similar to the
/// Fourier transform evaluated at the point $$\omega=0$$:
///
/// $$\int_{-\infty}^{\infty}p(x)dx = \hat{p}[\omega=0]$$
///
/// The only difference between the two is the upper limit of integration. We can now use a trick involving unit step functions. If you take an integral and multiply it by a unit step function, it has the effect of simply changing the limit of integration since the function is zero
/// outside that limit of integration and one within it. Therefore
///
/// $$\int_{-\infty}^{\infty}p(x)u(t-x)dx = \int_{-\infty}^{t}p(x)dx$$
///
/// Therefore, if we can multiply the probability distribution by a unit step function and evaluate the resulting Fourier transform at $$\omega=0$$, we can calculate the integral. Due to the convolution theorem, multiplication by a unit step function in the time domain
/// results in a convolution operation in the frequency domain. Convolution is computationally expensive, so we wish to avoid it as much as possible. Luckily, we only need to evaluate the Convolution of the distribution and the unit step function at a single point, $$\omega=0$$. Therefore, we do not need to compute the whole convolution, saving many computational resources.
///
/// From the definition of the discrete convolution,
///
/// $$(\hat{p}[x]*\hat{u}[x])[\omega=0] = \sum_{k=0}^{N}p[k]u[0-x] = \sum_{k=0}^{N}p[k]u[-x]$$
///
/// the `probability_below` function uses this methodology to compute the CDF of the distribution when desired
///

#[derive(Clone)]
pub struct ProbabilityDistribution {

    max_duration: f32, // the maximum simulated time duration
    omega_naught: f32, // the fundamental frequency of the fourier transform of the distribution

    transform: Vec<Complex<f32>>, // Fourier transform of the function

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
    /// when the duration of the run is close to the maximum duration of the run, or a multiple of it.
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

            // only add splits to the distribution that use the specified timing method (Real time vs In-Game time)
            match split.1[method] {
                Some(time) => {
                    // on the very first segment, there is no weighing, so we just insert the time with no weight
                    if history_index == 0 {
                        dist.transform = delta_function_dft(dist.omega_naught, num_terms, time.total_seconds() as f32);
                    }
                    else {
                        dist.add_point(time.total_seconds() as f32, smoothing_factor);
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        // println!("after adding the points");
        // println!("{:?}", dist.transform);

        return dist;

    }

    ///
    /// Adds a point to the probability distribution
    ///
    /// # Arguments
    ///
    /// * `time` - the time which is to be added to the distribution.
    /// * `smoothing_factor` - Smoothing factor of the exponential smoothing applied to the SegmentHistory (See [here](https://en.wikipedia.org/wiki/Exponential_smoothing))
    ///
    pub fn add_point(&mut self, time: f32, smoothing_factor: f32){

        let num_terms = self.transform.len();

        let dft = delta_function_dft(self.omega_naught, num_terms, time);

        // add the two vectors element wise in the form of an exponentially weighted average
        for frequency_index in 0..num_terms {
            self.transform[frequency_index] = dft[frequency_index] * smoothing_factor + (1.0 - smoothing_factor) * self.transform[frequency_index];
        }
    }

    ///
    /// Computes the Probability of the distribution being less than the specified amount (i.e. the
    /// Cumulative Distribution Function or CDF).
    ///
    /// # Arguments
    ///
    /// * `time` - The point in time below which we wish to know the probability of being
    ///
    /// # Mathematics
    ///
    /// the Cumulative Distribution Function is defined as the integral from -∞ to t of the
    /// probability density function (PDF)
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

    ///
    /// Obtains co-ordinate pairs to plot the probability density function
    ///
    pub fn plot(self) -> (Vec<f32>, Vec<f32>) {

        let num_terms = self.transform.len();

        let x_points = (0..num_terms).map(|x| x as f32 * self.max_duration / num_terms as f32).collect();

        // use the inverse fft we own to transform our points
        let mut inverted = self.transform.clone();
        self.fft_inverse.process(&mut *inverted);

        // take the magnitude of the y points to convert them to real numbers
        let y_points = inverted.iter().map(|x| x.abs() / num_terms as f32).collect();

        return (x_points, y_points);
    }

}

impl<'a, 'b> ops::Add<&'b ProbabilityDistribution> for &'a ProbabilityDistribution {
    type Output = ProbabilityDistribution;

    ///
    /// Computes the distribution of the sum of two random variables by convolution.
    ///
    /// By the Convolution theorem, convolution in the time domain corresponds to multiplication
    /// in the Frequency domain. Therefore, it is only necessary to do an element wise multiplication
    /// of the fourier transforms.
    ///
    fn add(self, other: &'b ProbabilityDistribution) -> ProbabilityDistribution {
        // copy self
        let mut result = self.clone().to_owned();

        // multiply the elements
        for i in 0..self.transform.len() {
            result.transform[i] *= other.transform[i];
        }

        return result;
    }

}

impl ops::AddAssign<ProbabilityDistribution> for ProbabilityDistribution {

    fn add_assign(self: &mut ProbabilityDistribution, rhs: ProbabilityDistribution){
        for i in 0..self.transform.len() {
            self.transform[i] *= rhs.transform[i];
        }
    }
}