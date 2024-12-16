fn main() {
    let bar = vec![1, 2, 3];
    let mut foo = bar.iter();
    println!("{:?}", foo.next());
    println!("{:?}", foo.next());
    println!("{:?}", foo.next());
    println!("{:?}", foo.next());
}
