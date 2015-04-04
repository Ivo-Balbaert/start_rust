fn main() {
	let hero1 = "Pac Man";
	let hero2 = "Riddick";
	greet(hero2);
	greet_both(hero1, hero2);

	let power = increment_power(1);
	println!("I am now at power level: {}", power);
	assert_eq!(2, power);
}

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