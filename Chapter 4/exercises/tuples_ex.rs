fn main() {
	// (2, 'a') == (3.14, false)
	// error: mismatched types: expected `()`, found `bool` (expected (), found bool)
	if () == () {
		println!("The unit value is an empty tuple");
	}
}
// he unit value is an empty tuple