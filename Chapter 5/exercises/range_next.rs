fn main() {
	let mut rng = 0..7;

	loop {
		match rng.next() {
			Some(val) => print!("{} - ", val),
			None	  => break
		}
	}
}
// 0 - 1 - 2 - 3 - 4 - 5 - 6 -