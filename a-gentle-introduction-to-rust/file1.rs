use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
	let first = env::args().nth(1).expect("Please supply a filename");

	let mut file = File::open(&first).expect("Cannot open the file.");

	let mut text = String::new();
	file.read_to_string(&mut text).expect("Cannot read the file");

	println!("File had {} bytes", text.len());
}
