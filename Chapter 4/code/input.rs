use std::io;

fn main() {
	println!("What's your name, noble warrior?");
	let mut buf = String::new();
	io::stdin().read_line(&mut buf)
			   .ok()
		       .expect("Failed to read line");
	let name = buf.trim();
	println!("{}, that's a mighty name indeed!", name);
}
// What's your name, noble warrior?
// Riddick
// Riddick, that's a mighty name indeed!

