#[macro_use]
extern crate log;
extern crate mac;

extern crate monsters;
use monsters::Monster; // import the trait

fn main() {
	monsters::print_from_monsters();
	let zmb1 = monsters::Zombie {health: 75, damage: 15}; 
	println!("Oh no, I hear: {}", zmb1.noise());
	zmb1.attack();
	println!("{:?}", zmb1);

	info!("Gathering information from monster {:?}", zmb1);
}
// Printing from crate monsters!
// Oh no, I hear: Aaargh!
// The Zombie bites! Your health lowers with 30 damage points.
// Zombie { health: 75, damage: 15 }