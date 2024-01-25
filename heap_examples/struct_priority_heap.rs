use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Define a custom struct
#[derive(Debug, PartialEq, Eq)]
struct MyStruct {
	name: String,
	value: i32,
}

// Implement Ord and PartialOrd for MyStruct based on the priority (value, in this case)
impl Ord for MyStruct {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.value.cmp(&other.value)
	}
}

impl PartialOrd for MyStruct {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

fn main() {
	// Creating a min heap of tuples (priority, struct)
	let mut min_heap: BinaryHeap<(Reverse<i32>, MyStruct)> = BinaryHeap::new();

	// Insert elements into the heap
	min_heap.push((Reverse(10), MyStruct { name: "A".to_string(), value: 10}));
	min_heap.push((Reverse(5), MyStruct { name: "B".to_string(), value: 5}));
	min_heap.push((Reverse(15), MyStruct { name: "C".to_string(), value: 15}));

	if let Some((Reverse(min_priority), min_element)) = min_heap.pop() {
		println!("Min Priority: {}, Min Element: {:?}", min_priority, min_element);
	}
}
