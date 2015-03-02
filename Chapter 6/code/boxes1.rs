struct Alien {
	planet: String,
	no_tentacles: u32,
}

fn main() {
	let mut a1 = Box::new(Alien{ planet: "Mars".to_string(), no_tentacles: 4 });
	println!("{}", a1.no_tentacles); // 4

	let a2 = &mut a1;
	println!("{}", a2.planet ); // Mars
	a2.no_tentacles = 5;
	// error: cannot borrow `a1.no_tentacles` as immutable because `a1` is also borrowed as mutable
	// println!("{}", a1.no_tentacles);
	// error: cannot assign to `a1.planet` because it is borrowed
	// a1.planet = "Pluto".to_string();

	// putting simple values on the heap:
	let n = Box::new(42);
	println!("{}", n); // 42
	// *n = 67; // error: cannot assign to immutable `Box` content `*n`
	let p = *n;
	println!("{}", p); // 42
	
	// p = 67; // error: re-assignment of immutable variable `p`
	// this is allowed:
	// let mut p = *n;
	// p = 67
	
	// another reference to n:
	let q = &*n;
	println!("{}", q); // 42
	println!("{}", square(q)); // 1764
}

fn square(k: &i32) -> i32 {
    *k * *k
}