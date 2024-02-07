extern crate rug;

use rug::Integer;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut n = Integer::from(0);

    loop {
        let f = factorial(&n);
        let digits = f.to_string();
        write_to_csv(n.to_string(), &digits);
        n += 1;
    }
}

fn factorial(n: &Integer) -> Integer {
    if n == &Integer::from(0) {
        return Integer::from(1);
    }

    let mut result = Integer::from(1);
    for i in 1..=n.to_u64().unwrap() {
        result *= &Integer::from(i);
    }
    result
}

fn write_to_csv(n: String, digits: &str) {
    let digits = &digits.chars().take(1000).collect::<String>();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("factorial_data.csv")
        .expect("Unable to open or create file");

    writeln!(file, "{},{}", n, digits)
        .expect("Unable to write data to file");
}

