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
	fn copy(&self) -> Self {
		Self::new(&self.first_name, &self.last_name)
	}
}


fn main() {
	let p = Person::new("Galen", "Seilis");
	let q = p.copy();
	println!("Person: {} {}", p.first_name, p.last_name);
	println!("Person: {} {}", q.first_name, q.last_name);
}