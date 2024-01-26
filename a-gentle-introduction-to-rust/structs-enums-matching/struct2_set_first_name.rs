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
	fn set_first_name(&mut self, name:&str) {
		self.first_name = name.to_string();
	}
}


fn main() {
	let mut p = Person::new("Galen", "Seilis");
	println!("Person: {} {}", p.first_name, p.last_name);
	p.set_first_name("Bob");
	println!("Person: {} {}", p.first_name, p.last_name);
}
