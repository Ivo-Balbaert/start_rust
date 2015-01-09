static MAXHEALTH: i32 = 100;
static GAMENAME: &'static str = "Monster Attack";

fn main() {
	println!("The Game you are playing is called {}.", GAMENAME);
	println!("You start with {} health points.", MAXHEALTH);
	println!("In the Game {0} you start with {1} % health, yes you heard it correct: {1} points!", 
		GAMENAME, MAXHEALTH);
	println!("You have {points} % health", points=70);

	// formatting:
	println!("MAXHEALTH is {:x} in hexadecimal", MAXHEALTH);
	println!("MAXHEALTH is {:b} in binary", MAXHEALTH);
}
// The Game you are playing is called Monster Attack.
// You start with 100 health points.
// In the Game Monster Attack you start with 100 % health, yes you heard it correct: 100 points!
// You have 70 % health
// MAXHEALTH is 64 in hexadecimal
// MAXHEALTH is 1100100 in binary