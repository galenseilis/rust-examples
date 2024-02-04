fn main() {
	let _v: () = ();

	let v = (2, 3);
	assert_eq!(explicitly_ret_unit(), implicitly_ret_unit());

	println!("Success!");
}

fn implicitly_ret_unit() {
	println!("I will return a ()");
}

fn explicitly_ret_unit() -> () {
	println!("I will return a ()");
}
