fn main() {
	let points = 10i32;
	let mut saved_points: u32 = 0;
    // saved_points = points; // error
    // error: mismatched types: expected `u32`, found `i32` (expected u32, found i32)
    saved_points = points as u32;
}