fn main() {
	let (x, y);
	(x,..) = (3,4);
	[.., y] = [1, 2];
	assert_eq!([x,y], [x,y]);

	println!("Success!");
}
