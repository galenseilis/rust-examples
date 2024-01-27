#[derive(Debug)]
enum Value {
	Number(f64),
	Str(String),
	Bool(bool)
}

fn eat_and_dump(v: Value) {
	use Value::*;
	match v {
		Number(n) => println!("Number is {}", n),
		Str(s) => println!("String is {}", s),
		Bool(b) => println!("Boolean is {}", b)
	}
}

fn main () {
    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);
	eat_and_dump(n);
	eat_and_dump(s);
	eat_and_dump(b);
}
