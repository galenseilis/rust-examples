// This should give a E0277 error

fn dump<T> (value: &T) {
	println!("value is {:?}", value);
}

fn main() {
	let n = 42;
	dump(&n);
}
