struct Alien {
	planet: String,
	n_tentacles: u32,
}

fn main() {
	let mut klaatu = Alien{ planet: "Venus".to_string(), n_tentacles: 15 };

	{	
		let kl2 = &mut klaatu;
		kl2.n_tentacles = 14;
		println!("{} - {}", kl2.planet, kl2.n_tentacles); // Venus - 14
		kl2.n_tentacles = 10;
	}

	println!("{} - {}", klaatu.planet, klaatu.n_tentacles); // Venus - 10
	klaatu.planet = "Pluto".to_string();
	println!("{} - {}", klaatu.planet, klaatu.n_tentacles); // Pluto - 10
}