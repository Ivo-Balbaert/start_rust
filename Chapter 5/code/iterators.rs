fn main() {
	// iterating over a range:
    let mut rng = 0..7;

    println!("> {:?}", rng.next()); // Some(0)
    println!("> {:?}", rng.next()); // Some(1)

    loop {
    	match rng.next() {
        	Some(x) => {
            	print!("{}", x);
        	},
        	None => { break }
    	}
	}
	println!("");

	// shorter way:
	let mut rng = 0..7;
    for n in rng {
    	print!("{} - ", n);
    } // 0 - 1 - 2 - 3 - 4 - 5 - 6 - 
    println!("");

	// iterating over arrays and vectors:
	let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
	println!("Here are the aliens: ");
	for alien in aliens.iter() {
		print!("{} / ", alien)
	}
	// shorter way:
	println!("Here are the aliens again: ");
	for alien in &aliens {
		print!("{} / ", alien)
	}
	println!("");

	println!("Here are the aliens in reverse: ");
	for alien in aliens.iter().rev() {
		print!("{} / ", alien)
	}

	// lazy iterator:
	let rng = 0..1000_000;
}
// > Some(0)
// > Some(1)
// 23456
// 0 - 1 - 2 - 3 - 4 - 5 - 6 - 
// Here are the aliens: 
// Cherfer / Fynock / Shirack / Zuxu / 
// Here are the aliens again: 
// Cherfer / Fynock / Shirack / Zuxu / 
// Here are the aliens in reverse: 
// Zuxu / Shirack / Fynock / Cherfer /