extern crate csv;
use std::io;

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    
    for result in rdr.records() {
        match result {
            Ok(record) => {
                // Use the CSV record here
                println!("{:?}", record);
            }
            Err(err) => {
                // Handle the error, e.g., print an error message
                eprintln!("Error reading CSV: {}", err);
            }
        }
    }
}

