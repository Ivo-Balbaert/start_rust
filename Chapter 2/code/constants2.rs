static MAX_HEALTH: i32 = 100;
static GAME_NAME: &'static str = "Monster Attack";

fn main() {
	const PI: f32 = 3.14;

	println!("The Game you are playing is called {}.", GAME_NAME);
	println!("You start with {} health points.", MAX_HEALTH);
	println!("In the Game {0} you start with {1} % health, yes you read it correctly: {1} points!", 
		GAME_NAME, MAX_HEALTH);
	println!("You have {points} % health", points=70);

	// formatting:
	println!("MAX_HEALTH is {:x} in hexadecimal", MAX_HEALTH);
	println!("MAX_HEALTH is {:b} in binary", MAX_HEALTH);
	println!( "Two written in binary is {0:b}", 2);

	println!("pi is {:e} in floating point notation", PI);

	let str = format!("You have {points} % health", points=70);
	println!("{}", str); // str now contains the value "You have 70 % health"
}
// The Game you are playing is called Monster Attack.
// You start with 100 health points.
// In the Game Monster Attack you start with 100 % health, yes you heard it correct: 100 points!
// You have 70 % health
// MAX_HEALTH is 64 in hexadecimal
// MAX_HEALTH is 1100100 in binary
// Two written in binary is 10
// pi is 3.14e0 in floating point notation
// You have 70 % health