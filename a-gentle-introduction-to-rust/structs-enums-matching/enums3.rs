enum Speed {
	Slow = 10,
	Medium = 20,
	Fast = 50
}

fn main() {
	let s = Speed::Slow;
	let speed = s as u32;
	println!("Speed {}", speed);
}
