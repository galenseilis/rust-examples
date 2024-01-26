trait Show {
	fn show(&self) -> String;
}

impl Show for i32 {
	fn show(&self) -> String {
		format!("Four-byte signed {}", self)
	}
}

impl Show for f64 {
	fn show(&self) -> String {
		format!("Eight-byte float {}", self)
	}
}

fn main() {
	let answer = 42;
	let maybe_pi = 3.14;
	let s1 = answer.show();
	let s2 = maybe_pi.show();
	println!("Show {}", s1);
	println!("Show {}", s2);
}
