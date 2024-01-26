struct Person {
	first_name: String,
	last_name: String
}

impl Person {
	fn new(first: &str, last: &str) -> Person {
		Person {
			first_name: first.to_string(),
			last_name: last.to_string()
		}
	}
}


fn main() {
	let p = Person::new("Galen", "Seilis");
	println!("Person: {} {}", p.first_name, p.last_name);
}
