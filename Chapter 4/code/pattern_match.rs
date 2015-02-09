use std::old_io;

fn main() {
	print!("Give a positive secret number: ");
	let input = old_io::stdin().read_line()
                           .ok()
                           .expect("Failed to read line");
    let input_num: Option<u32> = input.trim().parse();

    println!("Unwrap found {}", input_num.unwrap());

	match input_num {
        Some(num) => println!("{}", num),
        None      => println!("Please input an integer number!")
    };

    // binding the value of a match:
    let num = match input_num {
        Some(num) => num,
        None      => 0
    };

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
                        },
        (name, _, _) if name == "Thor" => println!("This is Thor!"),
        (_, _, pow)  if pow <= 1000    => println!("This is a powerless god"),
        _ => println!("This is something else")
    }
}
// Give a positive secret number: 42
// Unwrap found 42
// 42
// A good magician!
// It is contained in this range
// This is a demigod called Loki