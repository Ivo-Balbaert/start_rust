use std::io;

fn main() {
	println!("Give an integer number bigger than 0:");
	let mut buf = String::new();
	io::stdin().read_line(&mut buf)
	    .ok()
	    .expect("Failed to read line");
	let input_num: Result<u32, _>  = buf.trim().parse();
    let res = match input_num {
        Ok(num) => num,
        Err(_) => 0
    };
    println!("{}, that's a beautiful number", res);
}