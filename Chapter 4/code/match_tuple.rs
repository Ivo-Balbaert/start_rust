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

	let x = (59, false);
	match x {
		(n, true) if (n >= 20 && n <= 26) => println!("true and between 20 and 26"),
		(20...26, true) => println!("true and between 20 and 26"),
		(n, true) if !(n >= 20 && n <= 26) => println!("true and not between 20 and 26"),
		(40...49, _) => println!("between 40 and 49"),
		(_, _) => println!("Everything else")
	}

}
// Pyrrhic victory
// Everything else

