fn main() {
    let v = vec![1, 2, 3, 4];
    let squared: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("{:?}", squared);
}
