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

#[derive(Debug, Copy, Clone)]
struct MagicNumber {
    value: u64
}
// impl Copy for MagicNumber {}

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

	// copies because MagicNumber implements the Copy or Clone trait
	let mag = MagicNumber { value: 42 };
	let mag2 = mag;
	let mag3 = mag.clone();
	println!("{:?}", mag);
	println!("{:?}", mag2);
	// mag, mag2 and mag3 are 3 different objects: their addresses are different:
	println!("{:p}", &mag); // address is 0x23fb38
	println!("{:p}", &mag2); // address is 0x23fa80
	println!("{:p}", &mag3); // address is 0x23fb28
	// *const is a so called raw pointer, see Chapter 9                                   
    println!("{:?}", &mag as *const MagicNumber); // address is 0x23fb38
    println!("{:?}", &mag2 as *const MagicNumber); // address is 0x23fb30
    println!("{:?}", &mag3 as *const MagicNumber); // address is 0x23fb28

    // move of struct value:
    let mag = MagicNumber2 { value: 108 };
    println!("{:p}", &mag); // address is 0x23f6e8
	let mag2 = mag;
	// println!("{:p}", &mag); // error: use of moved value 'mag'
	println!("{:p}", &mag2); // address is 0x23f658
}

fn life(m: u32) -> u32 {
	let o = m;
	o
}

fn transform<'a>(s: &'a str) { /* ... */ }
fn transform_without_lifetime(s: &str) { /* ... */ }

// fn return_magician<'a>() -> &'a Magician {
//   let mag = Magician { name: "Gandalf", power: 4625 };
//   &mag // error: `mag` does not live long enough
// }

// The value of n2 is 42, the same as n
// MagicNumber { value: 42 }
// MagicNumber { value: 42 }
// 0x23fb38
// 0x23fb30
// 0x23fb28