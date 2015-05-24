fn main() {
	// let status = 7;
	// error: mismatched types: 
	// expected `_`,  found `(_, _)` (expected integral variable, found tuple) [E0308]

	// correction:
	// let status = ('a', false);  'a' is not an integer
	let status = (42, true);
	match status {
		(0, true) => println!("Success"),
		(_, true) => println!("Pyrrhic victory"), // Any first value matches
		(_, _)    => println!("Complete loss") // Any pair of values will match
	}
}
// Pyrrhic victory


