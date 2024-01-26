fn makes_a() -> A {
	let string = "I'm a little string".to_string();
	A { s: &string }
}
