use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(10);
    heap.push(20);
    heap.push(5);

    // Access the heap's internal vector directly
    let vec = &mut heap.into_vec();

    // Safely modify the second element (next after the top element)
    if vec.len() > 1 {
        let next_elem = &mut vec[1];  // Access the second largest element
        *next_elem += 1;
        println!("Modified second element: {}", next_elem);
    }

    // To restore the heap property, you can re-heapify the vector
    vec.sort_by(|a, b| b.cmp(a)); // For a max-heap
}
