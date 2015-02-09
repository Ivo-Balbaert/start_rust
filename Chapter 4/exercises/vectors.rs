fn main() {
	let mut numbers: Vec<i32> = Vec::new();
	numbers.push_all(&[3, 666, 13]);
	for n in numbers.iter() {
		print!("{} - ", n)
	}  // 3 - 666 - 13 -
	println!("");

	let magic_numbers = [7i32, 42, 47, 45, 54];
	numbers.push_all(&magic_numbers);
	for n in numbers.iter() {
		print!("{} - ", n)
	}  // 3 - 666 - 13 - 7 - 42 - 47 - 45 - 54 -

	let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
	// numbers.push_all(&aliens);
	// error: mismatched types: expected `&[i32]`, found `&[&str; 4]` (expected i32, found &-ptr)

	// empty array:
	let mut empty: [i32; 0] = [];
	println!("{:?}", empty); // []
}