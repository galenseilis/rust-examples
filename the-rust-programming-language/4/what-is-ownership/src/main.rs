fn main() {

    { // s is not valid here; not yet declared
        let _s = "Hello"; // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    {
        let _s = String::from("hello");
    }

    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("{s}");
    }

    {
        let x = 5;
        let _y = x;
    }

    {
        let s1 = String::from("hello");
        let _s2 = s1;
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {s1}, s2 = {s2}");
    }

    {
        let x = 5;
        let y = x;
        println!("x = {x}, y = {y}");
    }

    {
        let  s = String::from("galen"); // s comes into scope

        takes_ownership(s); // s's value moves into the function
                            // and so is no longer valid here

        let x = 5; // x comes into scope
        
        makes_copy(x); // x would move into the function,
                       // but i32 is a Copy, so it's okay to still
                       // use x afterward

        fn takes_ownership(some_string: String) { // somee_string comes into scope
            println!("{some_string}");
        } // here, some_string goes out of scope and `drop` is called.
          // The backing memory is freed

        fn makes_copy(some_integer: i32) { // some_integer comes into scope
            println!("{some_integer}");
        } // Here, some_integer goes out of scope. Nothing special happens.

    }

    {
        let _s1 = gives_ownership(); // gives_ownership moves its return value into s1
        let _s2 = String::from("galen"); // s2 comes into scope
        let _s3 = takes_and_gives_back(_s2); // s2 is moved into 
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3

        fn gives_ownership() -> String {
            let some_string = String::from("yours");
            some_string
        }

        fn takes_and_gives_back(a_string: String) -> String {
            a_string
        }
    }

    {
        let s1 = String::from("galen");
        let (s2, len) = calculate_length(s1);
        println!("the length of '{s2}' is {len}.");

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len();
            (s, length)
        }
    }

    {
        let s1 = String::from("Hi!");
        let len = calculate_length(&s1);
        println!("The length of '{s1}' is {len}.");

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }

    {
        let mut s = String::from("hello");
        change(&mut s);
        
        fn change(some_string: &mut String) {
            some_string.push_str(", Galen");
        }
    }

    {
        let mut s = String::from("hello");
        {
            let _r1 = &mut s;
        }
        let _r2 = &mut s;
    }

    {
        let mut s = String::from("hello");
        let _r1 = &s;
        let _r2 = &s;
        let _r3 = &mut s;
    }

    {
        let mut s = String::from("Hello");

        let r1 = &s;
        let r2 = &s;
        println!("{r1} and {r2}");
        let r3 = &mut s;
        println!("{r3}")
    }

    {
        //dangling reference
        let reference_to_nothing = dangle();

        fn dangle() -> &String {
            let s = String::from("hello");
            s
        }
    }

}

