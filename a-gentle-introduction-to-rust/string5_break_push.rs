fn main() {
	let mut s = String::new(); // initially this is empty
	s.push("Hello");
	s.push(' ');
	s += "World!"; // The `+=` is short for `push_str`.
	s.pop(); // Remove last character

	assert_eq!(s, "Hello World");
}
