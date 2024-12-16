fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Use an iterator to filter and transform numbers
    let evens_squared: Vec<i32> = numbers
        .iter()             // Create an iterator over numbers
        .filter(|&x| x % 2 == 0) // Keep only even numbers
        .map(|x| x * x)     // Square each even number
        .collect();         // Collect results into a vector
    
    println!("{:?}", evens_squared); // Output: [4, 16]
}

