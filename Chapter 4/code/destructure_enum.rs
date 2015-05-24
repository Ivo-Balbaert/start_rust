use En::*;

struct St {
	f1: i32,
	f2: f32
}

enum En {
	Var1,
	Var2,
	Var3(i32),
	Var4(i32, St, i32)
} 

fn foo(x: En) {
	match x {
		Var1 => println!("first variant"),
		Var2 => println!("second variant"),
		Var3(..) => println!("third variant"),
		Var4(..) => println!("fourth variant")
	}	
}

fn foo2(x: &En) {
	match x {
		&Var1 => println!("first variant"),
		&Var2 => println!("second variant"),
		&Var3(5) => println!("third variant with number 5"),
		&Var3(x) => println!("third variant with number {} (not 5)", x),
		&Var4(3, St{ f1: 3, f2: x }, 45) => {
			println!("destructuring an embedded struct, found {} in f2", x)
		}
		_ => println!("other (Var4)")
		// cannot move out of borrowed content:
		// &Var4(_, v, _) => {
		// 	println!("Some other Var4 with {} in f1 and {} in f2", v.f1, v.f2)
		// }
		// // _ => println!("other (Var2)") // unreachable pattern
	}
}

fn main() {
	let en1 = En::Var3(42);
	// foo(en1); // third variant
	foo2(&en1); // third variant with number 42 (not 5)
	let st1 = St { f1: 3, f2: 10.0 };
	// let en2 = En::Var4(3, st1, 45);
	// foo2(&en2); // destructuring an embedded struct, found 10 in f2
	let en3 = En::Var4(3, st1, 42);
	foo2(&en3); // other (Var4)
}
// third variant with number 42 (not 5)
// other (Var4)