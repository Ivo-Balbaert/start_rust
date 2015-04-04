fn main() {
	let energy = 5; // value 5 is bound to variable energy
	// splitting declaration and initialization:
	// let _energy = 5; // no warning unused variable
	// let energy = 5u; // energy is now an unsigned integer
	let copy_energy = energy;
	println!("Your energy is {}", energy);
	let level_title = "Level 1";
	let dead = false;
	let magic_number = 3.14f32;
	// an _ can be used to separate the digits from the type to improve readability:
	// let magic_number = 3.14_f32; 
	let empty = (); // the value of the unit type ()

	// changing values:
	// energy = 25; // error: re-assignment of immutable variable `energy`
	let mut fuel = 34;
	fuel = 60;

	let n; // error: unable to infer enough type information about `_`; type annotations required
	// println!("n is: {}", n); // error: use of possibly uninitialized variable 
	n = -2;
	let n: i32;
	// let n: i32 = -2; // n is a binding of type i32 and the value -2
	let x = 42u8;
	let magic_number = 3.14f64;
}