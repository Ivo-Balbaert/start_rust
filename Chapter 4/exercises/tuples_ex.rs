fn main() {
	// (2, 'a') == (5, false);
	// error: mismatched types: expected `char`, found `bool` (expected char, found bool)

	// mutability of tuples:
	let thor = (4, 5.0, false, "hello");
	println!("{} - {} - {}", thor.0, thor.1, thor.2);
	// thor.0 = 42; // error: cannot assign to immutable anonymous field `thor.0`

	let mut thor = (4, 5.0, false, "hello");
	println!("{} - {} - {}", thor.0, thor.1, thor.2);
	thor.0 = 42; // ok!
	println!("{} - {} - {}", thor.0, thor.1, thor.2);

	let empty_tup = ();
	if () == () {
		println!("The unit value is an empty tuple");
	}


}
// 4 - 5 - false
// 4 - 5 - false
// 42 - 5 - false
// The unit value is an empty tuple