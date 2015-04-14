fn main() {
	// CONSUMERS:
	println!("CONSUMERS: ");
	// collect:
	let rng = 0..1000;
	let rngvec = rng.collect::<Vec<i32>>();
	println!("{:?}", rngvec);

	// find:
	let mut rng = 0..1000;
	let forty_two = rng.find(|n| *n >= 42);
	println!("{:?}", forty_two);  // Some(42)
	// find needs a mutable variable, and moves it
	
	// ADAPTERS:
	// filter:
	println!("ADAPTERS: ");
	println!("FILTER:");
	rng = 0..1000;
	let rng_even = rng.filter(|n| is_even(*n))
					  .collect::<Vec<i32>>();
	println!("{:?}", rng_even);

	// alternative without collect:
	let rng = 1..100;
	let rng_even = rng.filter(|n| is_even(*n));
	for x in rng_even {
		println!("{}", x);
	}

	// map:
	println!("MAP:");
	let rng = 0..1000;
 	let rng_even_pow3 = rng.filter(|n| is_even(*n))
 						   .map(|n| n * n * n)
					       .collect::<Vec<i32>>();
	println!("{:?}", rng_even_pow3);
	println!("TAKE:");
	let rng = 0..1000;
	let rng_even_pow3_first5 = rng.filter(|n| is_even(*n))
 						   		  .map(|n| n * n * n)
 						   		  .take(5)
					       		  .collect::<Vec<i32>>();
	println!("{:?}", rng_even_pow3_first5);
}

fn is_even(n: i32) -> bool {
	n % 2 == 0
}
// CONSUMERS:
// [0, 1, 2, 3, 4, ..., 999 ]
// Some(42)
// ADAPTERS:
// FILTER:
// [0, 2, 4, ..., 996, 998]
// MAP:
// [0, 8, 64, ..., 988047936, 994011992]
// TAKE:
// [0, 8, 64, 216, 512]