fn main() {
	let x = 2.0 * std::f64::consts::PI;

	let abs_difference = (x.cos() - 1.0).abs();

	println!("{} {}", x, abs_difference);

	assert!(abs_difference < 1e-10);
}
