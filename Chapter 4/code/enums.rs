use PlanetaryMonster::MarsMonster;

enum Compass {
	North, South, East, West
}

type species = &'static str;

enum PlanetaryMonster {
	VenusMonster(species, i32),
	MarsMonster(species, i32)
}

fn main() {
	let direction = Compass::West;
	let martian = PlanetaryMonster::MarsMonster("Chela", 42);
	let martian = MarsMonster("Chela", 42);

	// using enum values:
	// error: binary operation `==` cannot be applied to type `Compass`
	// if direction == Compass::East { 
	// 	println!("Go to the east");
	// }

	match direction {
        Compass::North   => println!("Go to the North!"),
        Compass::East    => println!("Go to the East!"),
        Compass::South   => println!("Go to the South!"),
        Compass::West    => println!("Go to the West!"),
    }
}
// Go to the West!