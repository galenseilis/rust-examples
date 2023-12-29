fn main() {
	let mut v1 = vec![10, 20, 30, 40];
	v1.pop();

	let mut v2 = Vec::new();
	for i in 1..4 {
		v2.push(i * 10);
	}

	assert_eq!(v1, v2);

	v2.extend(0..2);
	assert_eq!(v2, &[10, 20, 30, 0, 1]);
}
