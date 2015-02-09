fn main() { 
	println!("No tests are compiled, compile with rustc --test!"); 
}

#[test]
fn arithmetic() {
	if 2 + 3 == 5 {
		println!("You can calculate!");
	}
}

#[test]
fn badtest() {
	assert_eq!(6, 2 + 3);
}