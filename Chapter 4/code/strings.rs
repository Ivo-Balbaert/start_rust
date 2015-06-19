use std::string;
use std::str::FromStr;

fn main() {
// string slices of type &str
	let magician1 = "Merlin";
	// let magician2: &'static str = "Gandalf";
    let magician2: &str = "Gandalf";
	let greeting = "Hello, 世界!";
	println!("Magician {} greets magician {} with {}", 
				magician1, magician2, greeting);

// making a String
    let mut str1 = String::new();  // created empty, no allocation
    let mut str2 = String::with_capacity(25);  // created with 25 bytes allocated

// convert a string slice into a String
	let mut str3 = magician1.to_string();
	println!("{}", str3);	// Merlin

// get a string slice from a String
	let sl1 = &str3;
	println!("{}", sl1);	// Merlin

// turning a String into a slice with &*
	let strm = "Sauron".to_string();
    println!("{}", &*strm);
    
// comparing Strings and string slices:
    if &str3[..] == magician1 {
        println!("We got the same magician alright!")
    }
    // shorter:
	if &str3 == magician1 {
		println!("We got the same magician alright!")
	}

// building a String
	let c1 = 'Ɵ';  // character c1
	str1.push(c1);		  
	println!("{}", str1); // Ɵ
	str1.push_str(" Level 1 is finished - ");
	println!("{}", str1); // Ɵ Level 1 is finished - 
	str1.push_str("Rise up to Level 2");
	println!("{}", str1); // Ɵ Level 1 is finished - Rise up to Level 2

// concatenating strings:
    let st1 = "United ".to_string(); // st1 is of type String
    let st2 = "States";
    let country = st1 + st2;
    println!("The country is {}", country);
    let st3 = "United ".to_string();
    let st4 = "States".to_string();
    let country = st3 + &st4;
    println!("The country is {}", country);

// iterate over String:
// characters:
	for c in magician1.chars() {
        print!("{} - ", c);
    } // M - e - r - l - i - n -

    for c in magician1.bytes() {
        print!("{} - ", c);
    } // 77 - 101 - 114 - 108 - 105 - 110

    // taking a character via an index does not work:
    // error: the trait `core::ops::Index<_>` is not implemented for the type `str` [E0277]
    // println!("{}", magician1[1]);
    println!("{:?}", magician1.chars().nth(1)); // Some('e')

// words:
	for word in str1.split(" ") {	
        print!("{} / ", word);
    }
    println!("");

// changing a String:
	let str5 = str1.replace("Level", "Floor");
	println!("{}", str5); // Ɵ Level 1 is finished - Rise up to Level 2

// from_str:
    println!("{:?}", f64::from_str("3.6"));
    let number: f64 = f64::from_str("3.6").unwrap();
   
// string parameters:
	println!("Length of Merlin: {}", how_long(magician1));
	println!("Length of str1: {}", how_long(&str1));
	println!("Length of str1: {}", how_long(&str1[..]));

// escape characters and raw strings r"" or r#""#:
    println!("{}", "Ru\nst");
    println!("{}", r"Ru\nst");
    println!("{}", r#"Ru\nst"#);
}

fn how_long(s: &str) -> usize {	s.len() }

// Magician Merlin greets magician Gandalf with Hello, 世界!
// Merlin
// Merlin
// Sauron
// We got the same magician alright!
// We got the same magician alright!
// Ɵ
// Ɵ Level 1 is finished - 
// Ɵ Level 1 is finished - Rise up to Level 2
// The country is United States
// The country is United States
// M - e - r - l - i - n - 77 - 101 - 114 - 108 - 105 - 110 - Some('e')
// Ɵ / Level / 1 / is / finished / - / Rise / up / to / Level / 2 / 
// Ɵ Floor 1 is finished - Rise up to Floor 2
// Ok(3.6)
// Length of Merlin: 6
// Length of str1: 43
// Length of str1: 43
// Ru
// st
// Ru\nst
// Ru\nst