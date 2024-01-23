fn main() {
	let s1 = "Hello, Dolly".to_string();
	let mut rs1 = &s1;
	{
		let tmp = "Hello, World".to_string();
		rs1 = &tmp;
	}
	println!("ref {}", rs1);
}
