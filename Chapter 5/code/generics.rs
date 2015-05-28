use std::f32;

struct Person {
	name: &'static str,
	id:   i32
}

struct Pair<T> {
    first: T,
    second: T,
}

fn main() {
	// generic structs:
	let magic_pair: Pair<u32> = Pair { first: 7, second: 42 };
	let pair_of_magicians: Pair<&str> = Pair { first: "Gandalf", second: "Sauron" };
	let a = second(magic_pair);

	// using Option<T>
	let x: Option<i8> = Some(5);
	let pi: Option<f64> = Some(3.14159265359);
	let none: Option<f64> = None;
	let none2 = None::<f64>;
	let name: Option<&str> = Some("Joyce");
	// let magic: Option<f32> = Some(42); // error: mismatched types

	let p1 = Person{ name: "James Bond", id: 7 };
	let p2 = Person{ name: "Vin Diesel", id: 12 };
	let p3 = Person{ name: "Robin Hood", id: 42 };
	let op1: Option<Person> = Some(p1);
	let pvec: Vec<Person> = vec![p2, p3]; // type annotation is not necessary
	// let pvec = vec![p2, p3];
	
	// using Result<T, E>
	// let m = sqroot(42.0);
	let m = sqroot(-5.0);
	
	// match m {
	// 	Ok(sq) => println!("The square root of 42 is {}", sq),
	// 	Err(str) => println!("{}", str)
	// }

	let res = match m {
		Ok(sq) => sq,
		Err(_) => -1.0
	};
	println!("res is {}", res);
}

fn sqroot(r: f32) -> Result<f32, String> {
    // if r < 0.0 { 
    // 	return Err("Number cannot be negative!".to_string()); 
    // }
    assert!(r >= 0.0, "Number cannot be negative!");
    Ok(f32::sqrt(r))
}

fn second<T>(pair: Pair<T>) {
	pair.second;
}
// The square root of 42 is 6.480741
// mres is 6.480741
// for m == -5.0: Number cannot be negative!