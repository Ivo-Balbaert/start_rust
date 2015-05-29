struct Alien {
	planet: String,
	n_tentacles: u32,
}

fn main() {
	// mutability can be changed by transfering ownership:
	let n = Box::new(42);
	let mut m = n;
	*m = 67;
	// println!("{}", n); // error: use of moved value: `n`
	println!("{}", m); // 67
	
	let mut a1 = Box::new(Alien{ planet: "Mars".to_string(), n_tentacles: 4 });
	// a move occurs here because it is a Box type:
	let a2 = a1; 
	//println!("{}", a1.n_tentacles); // error: use of moved value: `a1.n_tentacles`
	// a2.n_tentacles = 5; // cannot assign to immutable field a2.n_tentacles
	println!("{}", a2.n_tentacles); // 4
	use_alien(a2);
	// use_alien2(&*a2);
	// println!("{}", a2.n_tentacles); // error: use of moved value: `a2.n_tentacles`

	// automatic dereferencing:
	let ua = Box::new([1, 2, 3]);
    println!("{}", ua[0]); // 1

}

fn use_alien(a: Box<Alien>) {
	println!("An alien from planet {} is freed", a.planet);
}

fn use_alien2(a: &Alien) {
	println!("An alien from planet {} is freed", a.planet);
}

struct Recurs {
    list: Vec<u8>,
    rec_list: Option<Box<Recurs>>
}

// 67
// 4
// An alien from planet Mars is freed
// 1