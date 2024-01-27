fn next(&self) -> Direction {
	use Direction::*;
	match *self {
		Up => Right,
		Right => Down,
		Down => left,
		Left => Up
	}
}

fn main() {
	let mut d = start;
	for _ in 0..8 {
	println!("D {:?}", d);
	d = d.next();
	}
