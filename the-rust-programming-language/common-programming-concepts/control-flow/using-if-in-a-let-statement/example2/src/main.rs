fn main() {
	let condition = true;
	let number = if condition { 5 } else { "six" } // Will cause an error due to mismatched types:wq
	println!("The value of the number is: {number}");
}
