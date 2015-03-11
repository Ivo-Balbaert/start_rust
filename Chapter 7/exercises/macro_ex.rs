macro_rules! mac2 {
    ($arg:expr) => (3 * $arg);
}

macro_rules! mac3 {
    ($arg:ident) => (let $arg = 42);
}

macro_rules! mac4 {
    ($arg:expr) => ( { 
    				print!("start - ");
    				print!("{} - ", $arg);
    				print!("end");
    });
}

fn main() {
	println!("{}", mac2!(5)); // 15
	println!("{}", mac2!(2 + 3)); // 15
	mac3!(x); // expands into let x = 42;
    println!("{}", x); // 42
    mac4!("Where am I?"); // start - Where am I? - end
}