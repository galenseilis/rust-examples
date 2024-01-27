fn sqr<T> (x: T) -> T::Output
where T: std::ops::Mul + Copy {
	x * x
}

fn main() {
	println!("{}", sqr(12.0));
}
