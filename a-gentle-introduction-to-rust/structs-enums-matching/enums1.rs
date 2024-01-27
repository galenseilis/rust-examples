impl Direction {
	fn as_str(&self) -> &'static str {
		match *self { // This means taht *self will have type `Direction`.
			Direction::Up => "Up",
			Direction::Down => "Down",
			Direction::Left => "Left",
			Direction::Right => "Right"
		}
	}
}
