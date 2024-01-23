fn dump(s: &str) {
	println!("{}", s);
}

fn main() {
	let s1 = "Hello, Dolly".to_string();
	dump(&s1);
	println!("s1 {}", s1);
}
