use std::io;

fn main() {
	println!("What's your name, noble warrior?");
	let inp_name = io::stdin().read_line()
						  .ok()
						  .expect("Failed to read line");
	let name = inp_name.trim();
	println!("{}, that's a mighty name indeed!", name);
}