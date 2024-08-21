use num_bigint::BigUint;
use num_traits::One;
use std::env;

fn factorial(n: u64) -> BigUint {
    let mut result = BigUint::one();
    for i in 1..=n {
        result *= i;
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let num: u64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number");
            std::process::exit(1);
        }
    };

    let result = factorial(num);
}

