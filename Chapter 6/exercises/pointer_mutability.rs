fn main() {
	// through the mutable pointer the value of x has been changed:
	let mut x = 5;
	increment(&mut x);
	println!("x is now {}", x);

	// borrowed references cannot inherit mutability:
	// mut borrowed means: the reference can change, but not its value!
	let mut val1 = 10;
    let mut val2 = 20;
    let mut borrowed = &val1;
    borrowed = &val2;
    // *borrowed = 11; // error: cannot assign to immutable borrowed content `*borrowed`

    // references are type checked:
	let mut val1 = 10;
    let mut val3 = 10.0;
    let mut borrowed = &val1;
// error: mismatched types:
// expected `&_`, found `&_` (expected integral variable, found floating-point variable) [E0308]
    // borrowed = &val3; 
}

fn increment(r: &mut isize) {
    *r += 1;
    println!("r is now {}", *r);
}
// r is now 6
// x is now 6
