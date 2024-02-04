extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Read the path of the JSON file from standard input
    let mut file_path = String::new();

    println!("Enter the path of the JSON file:");
    io::stdin().read_line(&mut file_path).expect("Failed to read input");

    // Trim whitespace and newline characters from the input
    let file_path = file_path.trim();

    // Open and read JSON from the specified file
    let mut file_content = String::new();

    match File::open(file_path) {
        Ok(mut file) => {
            match file.read_to_string(&mut file_content) {
                Ok(_) => {
                    // Use Result to handle potential errors during parsing
                    match serde_json::from_str::<Value>(&file_content) {
                        Ok(v) => {
                            // Access the "name" field from the JSON
                            match v.get("name") {
                                Some(name) => {
                                    println!("Name: {}", name);
                                }
                                None => {
                                    println!("Name field not found in JSON.");
                                }
                            }
                        }
                        Err(err) => {
                            // Handle the error, e.g., print an error message
                            eprintln!("Error parsing JSON: {}", err);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
}

