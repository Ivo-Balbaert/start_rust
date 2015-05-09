use std::string;

fn main() {
// string slices of type &str
	let magician1 = "Merlin";
	let magician2: &'static str = "Gandalf";
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

// iterate over String:
// characters:
	for c in magician1.chars() {
        print!("{} - ", c); 
    } // M - e - r - l - i - n -
// words:
	for word in str1.split(" ") {	
        print!("{} / ", word);
    }
    println!("");

// changing a String:
	let str5 = str1.replace("Level", "Floor");
	println!("{}", str5); // Ɵ Level 1 is finished - Rise up to Level 2

// string parameters:
	println!("Length of Merlin: {}", how_long(magician1));
	println!("Length of str1: {}", how_long(&str1));
	println!("Length of str1: {}", how_long(&str1[..]));
}

fn how_long(s: &str) -> usize {	s.len() }


// Magician Merlin greets magician Gandalf with Hello, 世界!
// Merlin
// Merlin
// Sauron
// We got the same magician alright!
// Ɵ
// Ɵ Level 1 is finished - 
// Ɵ Level 1 is finished - Rise up to Level 2
// M - e - r - l - i - n - Ɵ / Level / 1 / is / finished / - / Rise / up / to / Level / 2 / 
// Ɵ Floor 1 is finished - Rise up to Floor 2
// Length of Merlin: 6
// Length of str1: 43
// Length of str1: 43
// Length of str1: 43