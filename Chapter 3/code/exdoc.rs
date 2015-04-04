fn main() {
	println!("The cube of 4 is {}", cube(4));
}

/// Calculates the cube `val * val * val`.
///
/// # Examples
///
/// ```
/// let cube = cube(val);
/// ```
pub fn cube(val: u32) -> u32 {
    // implementation goes here
    val * val * val
}
// The cube of 4 is 64