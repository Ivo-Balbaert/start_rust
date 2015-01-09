fn main() {
	// declarative statements:
	let a = 2;
	let b = 5;
	let n = a + b; // n binds to 7
	let m: i8;
	
	m = 42;	// expression that returns the unit value ()

	// expression that returns a + b
	let n1 = {
		let a = 2;
		let b = 5;
		a + b    // <-- no semicolon!
	};
	println!("n1 is: {}", n1);  // n1 is 7

	// expression that returns the unit value ()
	let n2 = {
		let a = 2;
		let b = 5;
		a + b;
	};
	println!("n2 is: {:?}", n2);  // n2 is ()
}
// 