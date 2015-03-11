pub fn print_from_monsters() {
	println!("Printing from crate monsters!");
}

pub trait Monster { 
    fn new(hlt: u32, dam: u32) -> Self;
    
    fn attack(&self);
    fn noise(&self) -> &'static str;
    
    fn attacks_with_sound(&self) {
        println!("The Monster attacks by making an awkward sound {}", self.noise());
    }
}

#[derive(Debug)]
pub struct Zombie { pub health: u32, pub damage: u32 } 

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

struct Alien { health: u32, damage: u32 }

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

struct Predator { health: u32, damage: u32 }
// Predator still has to implement new and noise methods:
// impl Monster for Predator {
// 	fn attack(&self) {
// 		println!("I bite you! Your health lowers with {} damage points.", 3 * self.damage);
// 	}
// }