// #![feature(box_syntax)]   // error: unstable feature in stable releases

fn main() {
	let health = 32;
	let mut game = "Space Invaders";
	println!("address of health-value: {:p}", &health); // prints 0x23fba4
	println!("address of game-value: {:p}", &game); // prints 0x23fb90
	println!("game-value: {}", game); // prints "Space Invaders"
	println!("game: {}", &game); // prints "Space Invaders"

	let game2 = &game;
	println!("{:p}", game2); // prints 0x23fb90
	println!("{}", *game2); // prints "Space Invaders"
	println!("{}", game2); // prints "Space Invaders"

	let x: &i64;
	// println!("{:?}", x); // error: use of possibly uninitialized variable: `x`
	
	// health = 33; // error: re-assignment of immutable variable `health`
	let y = &health;
	// now *y is the value 32

	// references to an immutable variable:
	let tricks = 10;
	// let reftricks = &mut tricks; // error: cannot borrow immutable local variable `tricks` as mutable

	// references to a mutable variable:
	let mut score = 0;
	let score2 = &score;
	// *score2 = 5; // cannot assign to immutable borrowed content *score2

	let mut score = 0;
	let score3 = &mut score;
	*score3 = 5;

	// let score4 = &mut score;
	// error: cannot borrow `score` as mutable more than once at a time

	// boxing values onto the heap:
	let x = Box::new(5i32); 
	// let y = box 6; // error: box expression syntax is experimental; you can call `Box::new` instead.
	// or use feature gate #![feature(box_syntax)]
	// let x = box 5i32; 

}
// address of health-value: 0x23fb04
// address of game-value: 0x23faf0
// game-value: Space Invaders
// game: Space Invaders
// 0x23faf0
// Space Invaders
// Space Invaders