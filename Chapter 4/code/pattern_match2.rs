fn main() {
	// exhaustive match with _:
    let magician = "Gandalf";
	match magician {
    	"Gandalf" => println!("A good magician!"),
    	"Sauron"  => println!("A magician turned bad!"),
        _         => println!("No magician turned up!")
	}

	// matching several values in a branch:
    let magical_number: i32 = 42;
    match magical_number {
    	// Match a single value
        1 => println!("Unity!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("Ok, these are primes"),
        // Match an inclusive range
        // 40...42 => println!("It is contained in this range"),
        num @ 40...42 => println!("{} is contained in this range ", num),
        // Handle the rest of cases
        _ => println!("No magic at all!"),
    }

    // destructuring values and using guards:
    let loki = ("Loki", true, 800u32); 
    match loki {
        (name, demi, _) if demi => {
                            print!("This is a demigod ");
                            println!("called {}", name);
                        }
        (name, _, _) if name == "Thor" => println!("This is Thor!"),
        (_, _, pow)  if pow <= 1000    => println!("This is a powerless god"),
        _ => println!("This is something else")
    }
}
// A good magician!
// 42 is contained in this range
// This is a demigod called Loki