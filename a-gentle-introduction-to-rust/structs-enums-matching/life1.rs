// This code gives a lifetime error
#[derive(Debug)]
struct A {
	s: &str
}

fn main() {
	let a = A {s: "Meow woof bark"};
	println!("{:?}", a);
}
