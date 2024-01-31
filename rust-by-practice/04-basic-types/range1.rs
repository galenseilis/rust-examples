fn main() {
	let mut sum = 0;
	for _i in -3..2 {
		sum += 1
	}

	assert!(sum == 5);

	for c in 97..123 {
		println!("{}", c);
	}
}
