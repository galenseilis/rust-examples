// https://www.youtube.com/watch?v=MoqtsYLGCC4

struct User {
	name: String,
	email: String
}
impl User {
	fn new(name: &str) -> User {
		User {
			name: name.to_string(),
			email: format!("{}@googlemail.com", name)
		}
	}
}

fn main() {
	let user = User::new("Galen");
	println!("Hello, {}", user.name);
	println!("Your email is {}", user.email);
}
