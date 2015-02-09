fn main() {
	let magician = "Merlin";
	// does not compile:
	// error: the trait `core::ops::Index<_>` is not implemented 
	// for the type `collections::string::String` [E0277]
	// in Unicode: each character can be a variable number of bytes
	// println!("{}", (magician.to_string())[0]);
	let mut str = String::new();
	str.push_str("Gandalf");
	// does not compile:
	// error: the trait `core::ops::Index<_>` is not implemented 
	// for the type `collections::string::String` [E0277]
	// in Unicode: each character can be a variable number of bytes
	// println!("{}", str[3]);

	// solution: use an iterator:
	let greeting = "Hello, 世界!";
	println!("Bytes:");
	for c in greeting.bytes() {
    	print!("{} - ", c);
	}
	println!("");
	println!("Chars:");
	for c in greeting.chars() {
    	print!("{} - ", c);
	}
	println!("");
	println!("Graphemes:");
	for c in greeting.graphemes(true) {
    	print!("{} - ", c);
	}
}
// Bytes:
// 72 - 101 - 108 - 108 - 111 - 44 - 32 - 228 - 184 - 150 - 231 - 149 - 140 - 33 - 
// Chars:
// H - e - l - l - o - , -   - 世 - 界 - ! - 
// Graphemes:
// H - e - l - l - o - , -   - 世 - 界 - ! -