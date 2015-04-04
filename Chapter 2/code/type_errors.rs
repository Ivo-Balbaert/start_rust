fn main() {
	let score: i32 = 100;
	// score = "YOU WON!";
	// error: mismatched types: expected `i32`, found `&'static str` 
	// (expected i32, found &-ptr)
	let score = "YOU WON!";

	let player1 = "Rob";
	let player2 = "Jane";
	// let player3 = player1 + player2;
	// error: binary operation `+` cannot be applied to type `&str`
	let player3 = player1.to_string() + player2;
    println!("{}", player3);
	let player3 = format!("{}{}", player1, player2);
	println!("{}", player3);
}
// RobJane
// RobJane