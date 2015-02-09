fn main() {
	let hero1 = "Pac Man";
	let hero2 = "Riddick";
	greet(hero2);
	greet_both(hero1, hero2);

	let mut power = 1;
	power = increment_power(power);
	println!("I am now at power level: {}", power);
	assert_eq!(2, power);
}

/// Greets a game character as mighty.
/// # Arguments
/// ** `name` ** - A string slice that holds the name of the character
/// # Example:  greet("Riddick");
fn greet(name: &str) {
	println!("Hi mighty {}, what brings you here?", name);
}

fn greet_both(name1: &str, name2: &str) {
	greet(name1);
	greet(name2);
}

fn increment_power(power: i32) -> i32 {
	// if power < 100 { return 999; }
	println!("My power is going to increase:");
	// power + 1; // results in: error: not all control paths return a value
	// return power + 1 // poor style
	power + 1
}
// Hi mighty Riddick, what brings you here?
// Hi mighty Pac Man, what brings you here?
// Hi mighty Riddick, what brings you here?
// My power is going to increase:
// I am now at power level: 2