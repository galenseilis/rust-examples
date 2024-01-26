#[derive(Debug)]
struct A {
	s: &'static str // Static lifetime!
}

fn main () {
	let a = A { s: "Meow Woof Bark"};
	println!("{:?}", a.s);
}
