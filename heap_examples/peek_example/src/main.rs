use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(10);
    heap.push(20);
    heap.push(5);

    if let Some(&top) = heap.peek() {
        println!("Top element: {}", top);
    }
}
