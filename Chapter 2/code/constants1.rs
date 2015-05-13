use std::f32::consts;

static MAX_HEALTH: i32 = 100;
static GAME_NAME: &'static str = "Monster Attack";

fn main() {
	const PI: f32 = 3.14;
	// use the PI value from the standard library:
	println!("{}", PI);
	println!("{}", consts::PI);
}
// 3.14
// 3.141593