use std::iter::FromIterator;

#[derive(Debug)]
struct MyCollection(Vec<String>);

impl FromIterator<String> for MyCollection {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut collection = Vec::new();
        for item in iter {
            collection.push(item);
        }
        MyCollection(collection)
    }
}

fn main() {
    let words = vec!["let", "it", "go"];

    let my_collection: MyCollection = words
        .into_iter()
        .map(String::from) // Convert &str to String
        .collect();

    println!("{:?}", my_collection);
}
