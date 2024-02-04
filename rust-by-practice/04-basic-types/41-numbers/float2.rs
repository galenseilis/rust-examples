fn main() {
	let summation: f64 = 0.1 + 0.2;
	let epsilon = 1e-10;
	assert!((summation - 0.3).abs() < epsilon);

	println!("Success!");
}
