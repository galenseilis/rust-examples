use std::collections::BinaryHeap;
use std::cmp::Ord;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct MyStruct {
    priority: i32,
    name: String,
}

fn main() {
    let mut heap = BinaryHeap::new();

    // Pushing elements with different priorities
    heap.push(MyStruct { priority: 10, name: String::from("Ten") });
    heap.push(MyStruct { priority: 20, name: String::from("Twenty") });
    heap.push(MyStruct { priority: 5, name: String::from("Five") });

    // Let's say we want to modify an attribute of the second element without breaking the heap invariant
    let vec = &mut heap.into_vec();  // Get the internal vector

    // Suppose we modify the `name` field of the second element
    if vec.len() > 1 {
        let next_elem = &mut vec[1];  // Access the second element (next after the top)
        next_elem.name = String::from("Modified");
    }

    // Print the modified heap
    println!("{:?}", vec);
}
