use rand;
use rand_distr::{Poisson, Distribution};

fn main() {
	let poi = Poisson::new(2.0).unwrap();
	let v = poi.sample(&mut rand::thread_rng());
	println!("{} is from a Poisson(2) distribution", v);
}
