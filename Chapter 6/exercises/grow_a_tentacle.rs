struct Alien {
	planet: String,
	no_tentacles: u32,
}

fn main() {
	let mut klaatu = Alien{ planet: "Venus".to_string(), no_tentacles: 15 };
	println!("Klaatu first has  {} tentacles", klaatu.no_tentacles); // 15
	grow_a_tentacle(&mut klaatu);
	println!("Klaatu has now {} tentacles", klaatu.no_tentacles);	// 16
}

fn grow_a_tentacle(al: &mut Alien) {
    al.no_tentacles += 1;
} // al goes out of scope

// Klaatu has first 15 tentacles
// Klaatu has now 16 tentacles