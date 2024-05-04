use rand_distr::{Distribution, Exp};
use rand::thread_rng;

fn main() {
    // Parameters
    let lambda = 0.1; // Rate parameter for the exponential distribution
	let sample_size = 1_000_000;

    // Create a random number generator
    let mut rng = thread_rng();

    // Create an exponential distribution with the specified rate parameter
    let exp_distribution = Exp::new(lambda).unwrap();

    // Sample from the exponential distribution
	for _ in 0..sample_size {
    	let sample = exp_distribution.sample(&mut rng);
		}
}

