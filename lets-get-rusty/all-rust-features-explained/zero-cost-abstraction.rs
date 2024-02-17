// What you don't use, you don't pay for.
// What you do use, you couldn't hand code any better.

fn main() {
	let numbers = vec![5, 2, 8, 1, 3];

	// Using iterator abstraction to find the maximum element
	let max_element = numbers.iter().max();

	match max_element {
		Some(&max) => println!("Maximum element: {}", max),
		None => println!("The vector is empty"),
	}
}
