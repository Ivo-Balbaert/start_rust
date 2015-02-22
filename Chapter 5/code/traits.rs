struct Alien { health: u32, damage: u32 }

#[derive(Debug)]
struct Zombie { health: u32, damage: u32 }

struct Predator { health: u32, damage: u32 }

trait Monster {
    fn new(hlt: u32, dam: u32) -> Self;
    
    fn attack(&self);
    fn noise(&self) -> &'static str;
    
    fn attacks_with_sound(&self) {
        println!("The Monster attacks by making an awkward sound {}", self.noise());
    }
}

impl Monster for Alien {
	fn new(mut h: u32, d: u32) -> Alien {
		// constraints:
		if h > 100 { h = 100; }
		Alien {health: h, damage: d}
	}

	fn attack(&self) {
		println!("I attack! Your health lowers with {} damage points.", self.damage);
	}

	fn noise(&self) -> &'static str {
		"Shriek!"
	}
}

impl Monster for Zombie {
	fn new(mut h: u32, d: u32) -> Zombie {
		// constraints:
		if h > 100 { h = 100; }
		Zombie {health: h, damage: d}
	}

	fn attack(&self) {
		println!("The Zombie bites! Your health lowers with {} damage points.", 2 * self.damage);
	}

	fn noise(&self) -> &'static str {
		"Aaargh!"
	}
}

// Predator still has to implement new and noise methods:
// impl Monster for Predator {
// 	fn attack(&self) {
// 		println!("I bite you! Your health lowers with {} damage points.", 3 * self.damage);
// 	}
// }

fn main() {
	let zmb1 = Zombie {health: 75, damage: 15};
	println!("Oh no, I hear: {}", zmb1.noise());
	zmb1.attack();
	println!("{:?}", zmb1);

}
// Oh no, I hear: Aaargh!
// The Zombie bites! Your health lowers with 30 damage points.
// Zombie { health: 75, damage: 15 }