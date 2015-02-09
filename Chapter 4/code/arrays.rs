fn main() {
	// arrays:
	let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
	println!("{:?}", aliens); 
	// a typed version:
	// let aliens: [&str; 4] = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
	// a mutable version:
	// let mut aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
	let zuxus = ["Zuxu"; 3]; // ["Zuxu", "Zuxu", "Zuxu"];
	println!("The first item is: {}", aliens[0]);
	println!("The third item is: {}", aliens[2]);
	println!("The last item is: {}", aliens[aliens.len() - 1]);
	// println!("This item does not exist: {}", aliens[10]); // runtime error:
	// thread '<main>' panicked at 'index out of bounds: the len is 4 but the index is 10'
	// change an item, but only if array is mutable!
	// aliens[2] = "Facehugger"; // error: cannot assign to immutable indexed content `aliens[..]`
	println!("Here are the aliens: ");
	for ix in 0 .. aliens.len() {
		println!("Alien no {} is {}", ix, aliens[ix]);
	}
	for a in aliens.iter() { println!("The next alien is {}", a); }
		
	// vectors:
	let mut numbers: Vec<i32> = Vec::new();
	let mut magic_numbers = vec![7i32, 42, 47, 45, 54];
	numbers.push(magic_numbers[1]);
	numbers.push(magic_numbers[4]);
	assert_eq!([42, 54], numbers);
	let fifty_four = numbers.pop(); // fifty_four now contains 54
	assert_eq!([42], numbers);

	// slices:
	let slc = &magic_numbers[1..4]; // only the items 42, 47 and 45 
	assert_eq!([42, 47, 45], slc);

	println!("");
	// slice from a String:
	let location = "Middle-Earth";
	let part = &location[7..12];
	println!("{}", part); // Earth

	// collect:
	let magician = "Merlin";
	let mut chars: Vec<char> = magician.chars().collect();
    chars.sort();
    for c in chars.iter() {
    	print!("{} ", c);
    }
 }
// ["Cherfer", "Fynock", "Shirack", "Zuxu"]
// The first item is: Cherfer
// The third item is: Shirack
// The last item is: Zuxu
// Here are the aliens: 
// Alien no 0 is Cherfer
// Alien no 1 is Fynock
// Alien no 2 is Shirack
// Alien no 3 is Zuxu
// The next alien is Cherfer
// The next alien is Fynock
// The next alien is Shirack
// The next alien is Zuxu

// Earth
// M e i l n r 