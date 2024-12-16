use std::iter::FromIterator;

#[derive(Debug)]
struct MyCollection(Vec<i32>);

impl FromIterator<i32> for MyCollection {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut collection = Vec::new();
        for item in iter {
            collection.push(item);
        }
        MyCollection(collection)
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let my_collection: MyCollection = numbers.into_iter().collect();
    println!("{:?}", my_collection);
}
