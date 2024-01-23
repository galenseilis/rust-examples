// absolute alue of a floating-point number
fn abs(x: f64) -> f64 {
	if x > 0.0 {
		x // `return` is not needed here.
	} else {
		- x // `return` is not needed here.
	}
}

fn main() {
	let x = 13.0;
	println!("abs(13.0) = {}", abs(x));

	let y = -8.0;
	println!("abs(-8.0) = {}", abs(y));
}
