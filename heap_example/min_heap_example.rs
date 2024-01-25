use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
	let mut min_heap = BinaryHeap::new();
	min_heap.push(Reverse(10));
	min_heap.push(Reverse(5));
	min_heap.push(Reverse(15));
	min_heap.push(Reverse(3));

	if let Some(Reverse(min_element)) = min_heap.pop() {
		println!("Min Element: {}", min_element);
	}
}
