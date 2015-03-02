struct Alien {
	planet: String,
	no_tentacles: u32,
}

fn main() {
	let mut klaatu = Alien{ planet: "Venus".to_string(), no_tentacles: 15 };

	// Question 2) - with the following statement:
	// let klaatuc = klaatu;
	// we get the following error at the kl2 binding:
	// error: use of moved value: `klaatu`
	// `klaatu` moved here (line 10: let klaatuc = klaatu;) because it has type `Alien`, 
	// which is moved by default

	let kl2 = &klaatu;
	// error: cannot assign to immutable field `kl2.no_tentacles`
	// (*kl2).no_tentacles = 14;
	// error: cannot assign to immutable field `kl2.no_tentacles`
	// kl2.no_tentacles = 10;
	println!("{} - {}", kl2.planet, kl2.no_tentacles); // Venus - 15

	// Question 1) 
	// error: cannot assign to `klaatu.planet` because it is borrowed
	// klaatu.planet = "Pluto".to_string();
	println!("{} - {}", klaatu.planet, klaatu.no_tentacles); // Venus - 15
}
// Venus - 15
// Venus - 15