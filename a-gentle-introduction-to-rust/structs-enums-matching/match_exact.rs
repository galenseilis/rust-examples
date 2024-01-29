fn dump(v: &Value) {
	use Value::*;
	match *v {
		Number(n) => println!("Number is {}", n),
		Str(ref s) => println!("String is '{}'", s),
		Bool(b) => println!("Boolean is {}", b)
	}
}


