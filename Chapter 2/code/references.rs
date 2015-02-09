fn main() {
	let health = 32;
	let game = "Space Invaders";
	println!("address of health-value: {:p}", &health); // prints 0x23fba4
	println!("address of game-value: {:p}", &game); // prints 0x23fb90
	println!("game-value: {}", game); // prints "Space Invaders"

	let game2 = &game;
	println!("{:p}", game2); // prints 0x23fb90
	println!("{}", *game2); // prints "Space Invaders"
	println!("{}", game2); // prints "Space Invaders"

	let x: &i64;
	let y = &health;
	// now *y is the value 32

	// game = "TicTacToe"; // error: re-assignment of immutable variable `game`

	let mut score = 0;
	let score2 = &mut score;

	let tricks = 10;
	// let reftricks = &mut tricks; // error: cannot borrow immutable local variable `tricks` as mutable

	// let score3 = &mut score;
	// error: cannot borrow `score` as mutable more than once at a time
}