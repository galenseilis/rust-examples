fn dump<T> (value: &T)
where T: std::fmt::Debug {
	println!("Value is {:?}", value);
}

fn main() {
	let n = 2018;
	dump(&n);
}
