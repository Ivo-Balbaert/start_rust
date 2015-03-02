struct Alien {
	planet: String,
	no_tentacles: u32,
}

fn main() {
	let mut klaatu = Alien{ planet: "Venus".to_string(), no_tentacles: 15 };

	{	
		let kl2 = &mut klaatu;
		(*kl2).no_tentacles = 14;
		println!("{} - {}", kl2.planet, kl2.no_tentacles); // Venus - 14
		kl2.no_tentacles = 10;
	}

	println!("{} - {}", klaatu.planet, klaatu.no_tentacles); // Venus - 10
	klaatu.planet = "Pluto".to_string();
	println!("{} - {}", klaatu.planet, klaatu.no_tentacles); // Pluto - 10
}