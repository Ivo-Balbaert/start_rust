fn main() {
	// panics:
	let x = 3;
	// let y = 0;
	let y = 3;
	// x / y; // thread '<main>' panicked at 'attempted to divide by zero'
	// if y == 0 { panic!("Division by 0 occurred, exiting"); }
	println!("{}", div(x, y));  // returns 1 if y = 3
	
	// assert!(x == 5); // thread '<main>' panicked at 'assertion failed: x == 5'
	// assert_eq!(x, 5); // thread '<main>' panicked at 'assertion failed: (left: `3`, right: `5`)', 
	// unreachable!(); // thread '<main>' panicked at 'internal error: entered unreachable code'
}

fn div(x: i32, y: i32) -> f32 {
	// if y == 0 { panic!("Division by 0 occurred, exiting"); }
	assert!(y != 0, "Division by 0 occurred, exiting");
	(x / y) as f32
}