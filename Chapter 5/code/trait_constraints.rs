// if r < Float::zero()
// error: use of unstable library feature 'std_misc': unsure about its place in the world

use std::num::Float;

fn main() {
	println!("The square root of {} is {:?}", 42.0f32, sqroot(42.0f32) );
	println!("The square root of {} is {:?}", 42.0f64, sqroot(42.0f64) );
	// error: the trait `std::num::Float` is not implemented for the type `_` [E0277]
	// println!("The square root of {} is {:?}", 42, sqroot(42) );		
}

fn sqroot<T: Float>(r: T) -> Result<T, String> {
	if r < Float::zero() { 
		return Err("Number cannot be negative!".to_string()); 
 	}
    Ok(Float::sqrt(r))
}

// trait constraint written with where clause syntax:
fn sqroot2<T>(r: T) -> Result<T, String> where T: Float {
	if r < Float::zero() { 
		return Err("Number cannot be negative!".to_string()); 
 	}
    Ok(Float::sqrt(r))
}

// faulty version:
// fn sqroot<T>(r: T) -> Result<T, String> {
// 	if r < 0.0 { 
// 		return Err("Number cannot be negative!".to_string()); 
//  	}
//     Ok(Float::sqrt(r))
// }

// The square root of 42 is Ok(6.480741)
// The square root of 42 is Ok(6.480741)

// fn multc<T: Trait1, U: Trait1 + Trait2>(x: T, y: U) {}
// fn multc<T, U>(x: T, y: U) where T: Trait1, U: Trait1 + Trait2 {}