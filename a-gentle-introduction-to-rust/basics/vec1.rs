fn main() {
	let mut v = Vec::new();
	let k = 10;

	for i in 1..k+1 {
		v.push(i * 10);
	}


	let first = v[0]; // will panic if out-of-range
	let maybe_first = v.get(0);

	println!("v is {:?}", v);
	println!("first is {}", first);
	println!("maybe_first is {:?}", maybe_first);
}
