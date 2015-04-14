struct Alien {
	planet: String,
	n_tentacles: u32
}

fn main() {
	let mut klaatu = Alien{ planet: "Venus".to_string(), n_tentacles: 15 };

	// a move of the resource:
	// let kl2 = klaatu;
	// println!("{}", klaatu.planet); // use of moved value 'klaatu.planet'

	// a borrowing of the resource:
	let kl2 = &mut klaatu;
	kl2.n_tentacles = 14;
	println!("{} - {}", kl2.planet, kl2.n_tentacles); // Venus - 14
	
	// ownership is transferred, original owner cannot access or change:
	// error: cannot assign to `klaatu.planet` because it is borrowed
	// klaatu.planet = "Pluto".to_string();
	// error: cannot borrow `klaatu.planet` as immutable because `klaatu` is also borrowed as mutable
	// println!("{} - {}", klaatu.planet, klaatu.n_tentacles); 
}