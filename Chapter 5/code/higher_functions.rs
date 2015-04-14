fn main() {
	let mut strength = 26;
	println!("My tripled strength equals {}", triples(strength) ); // 78
	println!("My strength is still {}", strength); // 26
	strength = triples(strength);
	println!("My strength is now {}", strength); // 78

	// function taking another function as parameter:
	strength = again(triples, strength);
	println!("I got so lucky to turn my strength into {}", strength); // 702 (= 3 * 3 * 78)

	// closures instead of a named function:
	strength = 78;
	let triples = |n| { 3 * n }; // a closure
	// closures infer the types of their arguments and return value types
	strength = again(triples, strength);
	println!("My strength is now {}", strength); // 702

	// inlining:
	strength = 78;
	strength = again(|n| {3 * n} , strength); // an inline closure
	// simplified without { }:
	// strength = again(|n| 3 * n , strength); // an inline closure
	println!("My strength is now {}", strength); // 702

	// a closure has access to (captures) the variables in its scope:
	let x: i32 = 42;
    let print_add = |s| { 
    	println!("x is {}", x); 
    	x + s
    }; 
    // || {...} is a closure which takes no arguments
    // each closure has its own unique type
	let res = print_add(strength); // <- here the closure is called "x is 42"
	assert_eq!(res, 744); // 42 + 702

	// a moving closure:
	let m: i32 = 42;
    let print_add_move = move |s| { 
    	println!("m is {}", m); 
    	m + s
    };
    let res = print_add_move(strength); 
	assert_eq!(res, 744); // 42 + 702 

}

fn triples(s: i32) -> i32 {
	3 * s
}

// this function does exactly the same as function triples:
fn triplesref(s: &i32) -> i32 {
	3 * *s
}

fn again<F: Fn(i32) -> i32>(f: F, s: i32, ) -> i32 {
    f(f(s))
}

// Output:
// My tripled strength equals 78
// My strength is still 26
// My strength is now 78
// I got so lucky to turn my strength into 702
// My strength is now 702
// My strength is now 702
// x is 42
// m is 42