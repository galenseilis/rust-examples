const N: u32 = 1000;
fn main() {
    let mut multiples = Vec::<u32>::new();
    let mut i: u32 = 0;
    loop {
        i += 1;
        if i == N {
            break;
        }
        if (i % 3 == 0) | (i % 5 == 0) {
            multiples.push(i);
        }
    }
    let result: u32 = multiples.iter().sum();
    println!("{result}");
}
