struct Alien {
	planet: String,
	no_tentacles: u32,
}

fn main() {
	// mutability can be changed by transfering ownership:
	let n = Box::new(42);
	let mut m = n;
	*m = 67;
	// println!("{}", n); // error: use of moved value: `n`
	println!("{}", m); // 67
	
	let mut a1 = Box::new(Alien{ planet: "Mars".to_string(), no_tentacles: 4 });
	// a move occurs here because it is a Box type:
	let a2 = a1; 
	//println!("{}", a1.no_tentacles); // error: use of moved value: `a1.no_tentacles`
	// a2.no_tentacles = 5; // cannot assign to immutable field a2.no_tentacles
	println!("{}", a2.no_tentacles); // 4
	use_alien(a2);
	// use_alien2(&*a2);
	// println!("{}", a2.no_tentacles); // error: use of moved value: `a2.no_tentacles`
}

fn use_alien(a: Box<Alien>) {
	println!("An alien from planet {} is freed", a.planet);
}

fn use_alien2(a: &Alien) {
	println!("An alien from planet {} is freed", a.planet);
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
// 67
// 4
// An alien from planet Mars is freed