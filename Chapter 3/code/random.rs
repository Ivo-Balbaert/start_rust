use std::rand;

fn main() {
	println!("Give me some random numbers:");
	for _ in 0..5 {
		// let rnd = rand::random::<i32>();  // integers 32 bits
		// let rnd = rand::random::<u32>();  // positive integers 32 bits
		let rnd = (rand::random::<u32>() % 32) + 1; // numbers between 1 and 32 (not included)
    	print!("{} / ", rnd);
	}
}
// Give me some random numbers:
// 22 / 13 / 23 / 31 / 15 /
// i32: 1874430781 / -282478132 / -1443939860 / 165374235 / 656033060 / 
// u32: 3186189551 / 1866438091 / 2909375433 / 664397067 / 3305048196 /