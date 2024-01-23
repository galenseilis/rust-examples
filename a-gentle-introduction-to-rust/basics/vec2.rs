fn dump(arr: &[i32]) {
	println!("arr is {:?}", arr);
}

fn main() {
	let mut v = Vec::new();
	let k = 10;

	for i in 1..k+1 {
		v.push(i * 10);
	}

	dump(&v);

	let slice = &v[1..];
	println!("slice is {:?}", slice);
}
