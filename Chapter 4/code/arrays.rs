fn main() {
	// arrays:
	let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
	println!("{:?}", aliens);
	let aliens: [&str; 4] = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
		// a mutable version:
	let mut aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
	let zuxus = ["Zuxu"; 3]; // ["Zuxu", "Zuxu", "Zuxu"];
		// empty array:
	let mut empty: [i32; 0] = [];
	println!("{:?}", empty); // []
	println!("The first item is: {}", aliens[0]);
	println!("The third item is: {}", aliens[2]);
	println!("The last item is: {}", aliens[aliens.len() - 1]);
	println!("The last item is: {}", aliens.iter().last().unwrap());
	// println!("This item does not exist: {}", aliens[10]); // runtime error:
	// thread '<main>' panicked at 'index out of bounds: the len is 4 but the index is 10'
	
	// Pointers to arrays use automatic dereferencing:
	let pa = &aliens;
	println!("Third item via pointer: {}", pa[2]);

	// change an item, but only if array is mutable!
	// aliens[2] = "Facehugger"; // error: cannot assign to immutable indexed content `aliens[..]`
	println!("Here are the aliens: ");
	for ix in 0..aliens.len() {
		println!("Alien no {} is {}", ix, aliens[ix]);
	}

	for a in aliens.iter() { 
		println!("The next alien is {}", a); 
	}

	for a in &aliens { 
		println!("The next alien is {}", a); 
	}
	
	// making vectors:
	let mut numbers: Vec<i32> = Vec::new();
	// let mut numbers: Vec<_> = Vec::new();
	
	let mut magic_numbers = vec![7i32, 42, 47, 45, 54];
	
	let mut ids: Vec<i32> = Vec::with_capacity(25);
	
	let rgvec: Vec<u32> = (0..7).collect();
	println!("Collected the range into: {:?}", rgvec);

    // push and pop:
	numbers.push(magic_numbers[1]);
	numbers.push(magic_numbers[4]);
	println!("{:?}", numbers);
	let fifty_four = numbers.pop(); // fifty_four now contains 54
	println!("{:?}", numbers);

	// iterate over a vector:
	let values = vec![1, 2, 3];
	for n in values {
	    println!("{}", n);
	}

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
	let v: Vec<&str> = "The wizard of Oz".split(' ').collect();
	    assert_eq!(v, vec!["The", "wizard", "of", "Oz"]);

	let v: Vec<&str> = "abc1def2ghi".split(|c: char| c.is_numeric()).collect();
	    assert_eq!(v, vec!["abc", "def", "ghi"]);
	}
// ["Cherfer", "Fynock", "Shirack", "Zuxu"]
// []
// The first item is: Cherfer
// The third item is: Shirack
// The last item is: Zuxu
// The last item is: Zuxu
// Third item via pointer: Shirack
// Here are the aliens: 
// Alien no 0 is Cherfer
// Alien no 1 is Fynock
// Alien no 2 is Shirack
// Alien no 3 is Zuxu
// The next alien is Cherfer
// The next alien is Fynock
// The next alien is Shirack
// The next alien is Zuxu
// Collected the range into: [0, 1, 2, 3, 4, 5, 6]
// [42, 54]
// [42]
// 1
// 2
// 3

// Earth
// M e i l n r