// Should throw a value only lives until here if we appropriately put this in `main`.
fn makes_a() -> A<'static> {
	let string = "I'm a little string".to_string();
	A { s: &string }
}
