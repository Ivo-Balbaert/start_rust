struct Player {
    nname: &'static str,     // nickname
    health: i32,
    level: u8
}	

fn main() {
	struct Scoreu; // unit struct

	// tuple structs:
	struct Score(i32, u8);
	let score1 = Score(73, 2); // make (instantiate) a tuple struct
	let Score(h, l) = score1; // // extract values by destructuring
	println!("Health {} - Level {}", h, l);
	
	// newtype:
	struct Kilograms(u32);
	let weight = Kilograms(250);
	let Kilograms(kgm) = weight; // extracting kgm
	println!("weight is {} kilograms", kgm);

	// struct:
	let mut pl1 = Player{ nname: "Dzenan", health: 73, level: 2 };
	println!("Player {} is at level {}", pl1.nname, pl1.level);
	pl1.level = 3;

	// pointers do automatic dereferencing when accessing data structure elements:
	let ps = &Player{ nname: "John", health: 95, level: 1 };
    println!("{} == {}", ps.nname, (*ps).nname);

	// destructuring a struct:
	let Player{ health: ht, nname: nn, .. } = pl1;
	println!("Player {} has health {}", nn, ht);
}
// Health 73 - Level 2
// weight is 250 kilograms
// Player Dzenan is at level 2
// John == John
// Player Dzenan has health 73