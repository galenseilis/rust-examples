const N: u64 = 600851475143;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        } else if i * i >= n {
            return true;
        } else {
            continue ;
        }
    }
    return true;
}

fn main() {
    let mut largest = 2;
    for j in 1..N {
        if is_prime(j) {
            println!("{}", j);
        }
    }
}
