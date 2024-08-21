fn fib(n: u32) -> u32 {
    if n <= 1  {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}


fn main() {
    let mut result = 0;
    let mut i = 0;
    loop {
        i += 1;
        let fib_i = fib(i);
        if (fib_i % 2 == 0) && (fib_i <= 4000000) {
            result += fib_i;
        println!("{}", fib(i));
        } else if fib_i > 4000000 {
            break;
        }
    }
    println!("Result {}", result);
}
