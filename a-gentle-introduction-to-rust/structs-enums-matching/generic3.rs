// Will give a "the trait std::fmt::Debug is not implemented for `Foo`"
fn dump<T> (value: &T)
where T: std::fmt::Debug {
	println!("Value is {:?}", value);
}

struct Foo {
	name: String
}


fn main() {
	let foo = Foo {name: "Hello".to_string()};

	dump(&foo)
}

