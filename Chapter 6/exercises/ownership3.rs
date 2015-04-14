struct Alien {
	planet: String,
	no_tentacles: u32,
}

fn main() {
	let mut klaatu = Alien{ planet: "Venus".to_string(), no_tentacles: 15 };

	// Question 1) 
	let kl2 = &klaatu;
	// error: cannot assign to immutable field `kl2.no_tentacles`
	// kl2.no_tentacles = 14;
	println!("{} - {}", kl2.planet, kl2.no_tentacles); // Venus - 15

	// error: cannot assign to `klaatu.planet` because it is borrowed
	// klaatu.planet = "Pluto".to_string();
	println!("{} - {}", klaatu.planet, klaatu.no_tentacles); // Venus - 15

	// Question 2) - with the following statement:
	// let klaatuc = klaatu;
	// let kl2 = &klaatu;
	// we get the following error at the kl2 binding:
	// error: use of moved value: `klaatu`
	// `klaatu` moved here (line 10: let klaatuc = klaatu;) because it has type `Alien`, 
	// which is moved by default

	// mutability can be changed when ownership is transferred:
	let im = Box::new(7u32);
    // Mutability error:
    // error: cannot assign to immutable `Box` content `*im`
    // *im = 4;
    // Hand over the box, changing mutability
    let mut muta = im;
    println!("muta contains {}", muta);
    // println!("im contains {}", im); // error: use of moved value `im`
    // Modify the contents of the box
    *muta = 42;
    println!("muta now contains {}", muta);
}
// Venus - 15
// Venus - 15
// muta contains 7
// muta now contains 42