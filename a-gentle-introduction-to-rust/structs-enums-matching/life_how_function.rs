fn how(i: u32) -> &'static str {
	match i {
	0 => "none",
	1 => "one",
	_ => "many"
	}
}
