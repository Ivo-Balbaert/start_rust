#[derive(Debug)]
struct Zombie { health: u32, damage: u32 }

trait SuperMonster {
    fn test();
}

// this 'inheritance' implies that any type that implements trait Monster
// must also implement the trait SuperMonster  
trait Monster : SuperMonster {
    fn new(hlt: u32, dam: u32) -> Self;
    
    fn attack(&self);
    fn noise(&self) -> &'static str;
    
    fn attacks_with_sound(&self) {
        println!("The Monster attacks by making an awkward sound {}", self.noise());
    }
}

impl SuperMonster for Zombie {
    fn test() {
        println!("I am a SuperMonster");
    }
}

impl Monster for Zombie {
	fn new(mut h: u32, d: u32) -> Zombie {
		// constraints:
		if h > 100 { h = 100; }
		Zombie { health: h, damage: d }
	}

	fn attack(&self) {
		println!("The Zombie bites! Your health lowers with {} damage points.", 2 * self.damage);
	}

	fn noise(&self) -> &'static str {
		"Aaargh!"
	}
}

fn main() {
    // use Monster; // <- not necessary!
	let zmb1 = Zombie { health: 75, damage: 15 };
	println!("Oh no, I hear: {}", zmb1.noise());
	zmb1.attack();
	println!("{:?}", zmb1);
}
// Oh no, I hear: Aaargh!
// The Zombie bites! Your health lowers with 30 damage points.
// Zombie { health: 75, damage: 15 }