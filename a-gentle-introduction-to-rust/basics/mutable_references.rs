fn modifies(x: &mut f64) { // Note that we did not set a return type.
	*x = 1.0;
}

fn main() {
	let mut res = 0.0;
	println!("res was {}", res);
	modifies (&mut res);
	println!("res is {}", res);
}
