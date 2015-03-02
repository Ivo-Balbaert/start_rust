struct Alien {
	planet: String,
	no_tentacles: u32,
}

fn main() {
	let mut klaatu = Alien{ planet: "Venus".to_string(), no_tentacles: 15 };

	// a move of the resource:
	let kl2 = &mut klaatu;
	(*kl2).no_tentacles = 14;
	println!("{} - {}", kl2.planet, kl2.no_tentacles); // Venus - 14
	kl2.no_tentacles = 10;

	// ownership is transferred, original owner cannot access or change:
	// error: cannot assign to `klaatu.planet` because it is borrowed
	// klaatu.planet = "Pluto".to_string();
	// error: cannot borrow `klaatu.planet` as immutable because `klaatu` is also borrowed as mutable
	// println!("{} - {}", klaatu.planet, klaatu.no_tentacles); 
}