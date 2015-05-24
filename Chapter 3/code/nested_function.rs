fn main() {
	foo();
}

fn foo() {
	fn bar() { println!("bar"); }
	bar();
}
// bar
