fn factorial(n: u128) -> u128 {
	if n == 0 {
		1
	} else {
		n * factorial(n - 1)
	}
}

fn main () {
	for i in 0..35 {
		println!("{}! = {}", i, factorial(i));
	}
}
