fn main() {
	let thor = ("Thor", true, 3500u32);
	println!("{:?}", thor); // ("Thor", true, 3500u)
	// type of thor: (&str, bool, usize)
	println!("{} - {} - {}", thor.0, thor.1, thor.2);

	// destructuring:
	let (name, _, power) = thor;
	println!("{} has {} points of power", name, power);

	// one-element tuple:
	let one = (1,);

	// function returning a tuple and destructuring the return value:
	let (god, strength) = increase_power(thor.0, thor.2);
	println!("This god {} has now {} strength", god, strength);
}

fn increase_power(name: &str, power: u32) -> (&str, u32) {
	if power > 1000 {
		return (name, power * 3);
	} else {
		return (name, power * 2);
	}
}

// ("Thor", true, 3500u)
// Thor - true - 3500
// Thor has 3500 points of power
// This god Thor has now 10500 strength

