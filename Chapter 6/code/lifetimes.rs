struct Magician {
	name: &'static str,
	power: u32
}

// this code does not compile:
// error: missing lifetime specifier [E0106]
// struct MagicNumbers {
// 	magn1: &u32,
// 	magn2: &u32
// }

// this code is ok, both the struct and the fields have lifetime 'a:
struct MagicNumbers<'a> {
	magn1: &'a u32,
	magn2: &'a u32
}

#[derive(Debug, Clone)]
struct MagicNumber {
    value: u64
}
impl Copy for MagicNumber {}

#[derive(Copy)]
struct MagicNumber2 {
    value: u64
}

fn main() {
	// lifetimes restricted to a function:
	let n = 42u32;
	// copy behaviour:
	// no move, only a copy of the value from n to n2:
	let n2 = n;
	println!("The value of n2 is {}, the same as n", n2);

	life(n);
	// println!("{}", m);  // error: unresolved name `m`.
	// println!("{}", o);  // error: unresolved name `o`.

	// lifetime restricted to a code block:
	{
		let phi = 1.618;
	}
	// error: unresolved name `phi`.
	// println!("The value of phi is {}", phi);

//	let m = return_magician();
//	println!("{} has {}", m.name, m.power);

	let mag = MagicNumber {value: 42};
	let mag2 = mag;
	let mag3 = mag.clone();
	println!("{:?}", mag);
	println!("{:?}", mag2);
	// mag and mag2 are 2 different objects, their addresses are different:
	println!("{:?}", &mag as *const _); // address is 0x23fa88
    println!("{:?}", &mag2 as *const _); // address is 0x23fa80
    println!("{:?}", &mag3 as *const _); // address is 0x23fa78
}

fn life(m: u32) -> u32 {
	let o = m;
	o
}

fn transform<'a>(s: &'a str) { /* ... */ }
fn transform_without_lifetime(s: &str) { /* ... */ }

// fn return_magician<'a>() -> &'a Magician {
//   let mag = Magician { name: "Gandalf", power: 4625};
//   return &mag; // error: `mag` does not live long enough
// }