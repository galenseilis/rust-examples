extern crate rug;

use rug::Integer;

fn main() {
    let mut n = Integer::from(253817);

    loop {
        let f = factorial(&n);
        if count_unique_digits(&f.to_string()) != 10 {
            println!("{} {}", n, f);
        }
		println!("{}", n);
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

fn count_unique_digits(num_str: &str) -> usize {
    let mut digits = std::collections::HashSet::new();
    for ch in num_str.chars() {
        if ch.is_digit(10) {
            digits.insert(ch);
        }
    }
    digits.len()
}

