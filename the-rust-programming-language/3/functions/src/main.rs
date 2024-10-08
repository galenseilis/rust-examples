fn main() {
    { // Functions
        println!("Hello, world!");

        fn another_function() {
            println!("Another function.");
        }

        another_function();
    }

    { // Parameters
        another_function(5);

        fn another_function(x: i32) {
            println!("The value of x is: {x}");
        }
    }

    {
        print_labeled_measurement(5, 'h');

        fn print_labeled_measurement(value: i32, unit_label: char) {
            println!("The measurement is: {value}{unit_label}");
        }
    }

    { // Statements and expressions
        let _y = 6;
    }

    {
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {y}");
    }

    {
        fn five() -> i32 {
            5
        }

        let x = five();

        println!("The value of x is: {x}");
    }
    
    {
        let x = plus_one(5);

        println!("The value of x is: {x}");

        fn plus_one(x: i32) -> i32 {
            x + 1
        }
    }
}
