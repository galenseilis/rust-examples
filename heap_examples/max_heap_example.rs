use std::collections::BinaryHeap;

fn main() {
	let mut heap = BinaryHeap::new();

	heap.push(10);
	heap.push(5);
	heap.push(15);
	heap.push(3);

	if let Some(max_element) = heap.pop() {
		println!("Max Element: {}", max_element);
	}
}
