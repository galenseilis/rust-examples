fn is_palindrome(x: i32) -> bool {
    let s: String = x.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let num = 12321;
    if is_palindrome(num) {
        println!("{num} is a palindrome.");
    } else {
        println!("{num} is not a palindrome.");
    }
}
