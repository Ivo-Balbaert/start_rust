fn main() {
	let dead = false;
	let health = 48;

	if dead {
		println!("Game over!");
		return;
	}
	
	if dead {
		println!("Game over!");
		return;
	} else {
		println!("You still have a chance to win!");
	}

	if health >= 50 {
		println!("Continue to fight!");
	} else if health >= 20 {
		println!("Stop the battle and gain strength!");
	} else {
		println!("Hide and try to recover!");
	}

	let active = if health >= 50 {
					true
				 } else {
				 	false
				 };
	println!("Am I active? {}", active);

	// alternative for ternary operator:
	let adult = true;
	let age = if adult { "+18" } else { "-18" };
	println!("Age is {}", age);
}
// Stop the battle and gain strength!
// Am I active? false
// Age is +18