use std::iter;

fn main() {
	// iterating over a range:
    let mut rng = 0..7;

    println!("> {:?}", rng.next()); // Some(0)
    println!("> {:?}", rng.next()); // Some(1)

    for n in rng {
    	print!("{} - ", n);
    } // 2 - 3 - 4 - 5 - 6 -
    println!("");

	// iterating over arrays and vectors:
	let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
	println!("Here are the aliens: ");
	for alien in aliens.iter() {
		print!("{} / ", alien)
	}
	println!("");
	println!("Here are the aliens in reverse: ");
	for alien in aliens.iter().rev() {
		print!("{} / ", alien)
	}

	// lazy iterators:
	let rng = 0..1000_000;
	println!("");
	let mut inf = iter::count(42, 7);
	for _ in 0..3 {
		print!("Next: {:?} / ", inf.next());
	}
}
// > Some(0)
// > Some(1)
// 2 - 3 - 4 - 5 - 6 - 
// Here are the aliens:
// Alien no 0 is Cherfer
// Alien no 1 is Fynock
// Alien no 2 is Shirack
// Alien no 3 is Zuxu
// Cherfer / Fynock / Shirack / Zuxu / 
// Here are the aliens in reverse: 
// Zuxu / Shirack / Fynock / Cherfer / 
// Next: Some(42) / Next: Some(49) / Next: Some(56)