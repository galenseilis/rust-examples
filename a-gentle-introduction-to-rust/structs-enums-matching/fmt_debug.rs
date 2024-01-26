use std::fmt;

impl fmt::Debug for Person {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.full_name())
	}
}


// ...

	println!("{:?}", p);
