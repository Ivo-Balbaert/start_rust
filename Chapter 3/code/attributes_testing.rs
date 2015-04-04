fn main() { 
	println!("No tests are compiled, compile with rustc --test!"); 
}

#[test]
fn arithmetic() {
	if 2 + 3 == 5 {
		println!("You can calculate!");
	}
	if 2 + 3 == 6 {  // this test passes as wel!
		println!("You cannot calculate!");
	}
	// good tests:
	assert_eq!(5, 2 + 3);
	assert!(2 + 3 == 5);
}

#[test]
fn badtest() {
	assert_eq!(6, 2 + 3);
}