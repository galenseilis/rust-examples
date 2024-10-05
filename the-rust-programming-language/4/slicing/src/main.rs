#![allow(warnings)]
fn main() {
    {    
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }
            s.len()
        }

        let mut s = String::from("hello world");
        let _word = first_word(&s);
        s.clear()
    }
   
   {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
    }

    {
        let s = String::from("hello");

        let slice = &s[0..2];
        let slice = &s[..2];
    }
    {
        let s = String::from("hello");

        let len = s.len();
        let slice = &s[3..len];
        let slice = &s[3..];
    }
}

