fn verbose(x: i32) -> &'static str {
	let mut result: &'static str;
	if x < 10 {
		result = "less than 10";
	} else {
		result = "10 or more";
	}
	return result;
}

fn simple(x: i32) -> &'static str {
	if x < 10 { "less than 10" } 
	else { "10 or more"	}
}

fn main() {
	let n = 13;
	println!("verbose {}", verbose(n));
	println!("simple {}", simple(n));
}
// verbose 10 or more
// simple 10 or more
