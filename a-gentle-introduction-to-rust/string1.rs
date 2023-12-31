fn dump(s: &str) {
	println!("str '{}'", s);
}

fn main () {
	let text = "hello dolly"; // the string slice
	let s = text.to_string(); // it is now an allocated string.
	dump(text);
	dump(&s);
}
