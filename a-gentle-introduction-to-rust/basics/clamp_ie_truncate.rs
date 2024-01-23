// ensure the number always falls in the given range

fn clamp(x: f64, x1:f64, x2: f64) -> f64 {
	if x < x1 {
		x1
	} else if x > x2 {
		x2
	} else {
		x
	}
}

fn main () {
	let x: f64 = -3.14;
	let y: f64 = 3.14;
	let z: f64 = 13.14;

	println!("clamp(-3.14, 0, 10) = {}", clamp(x, 0.0, 10.0));	
	println!("clamp(3.14, 0, 10) = {}", clamp(y, 0.0, 10.0));
	println!("clamp(13.14, 0, 10) = {}", clamp(z, 0.0, 10.0));
}
