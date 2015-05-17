extern crate num;
use num::traits::Float;

fn main() {
	println!("The square root of {} is {:?}", 42.0f32, sqroot(42.0f32) );
	println!("The square root of {} is {:?}", 42.0f64, sqroot(42.0f64) );
}

fn sqroot<T: num::traits::Float>(r: T) -> Result<T, String> {
    if r < num::zero() { 
        return Err("Number cannot be negative!".to_string()); 
    }
    Ok(num::traits::Float::sqrt(r))
}

// trait constraint written with where clause syntax:
fn sqroot2<T>(r: T) -> Result<T, String> where T: num::traits::Float {
    if r < num::zero() { 
        return Err("Number cannot be negative!".to_string()); 
    }
    Ok(num::traits::Float::sqrt(r))
}
// The square root of 42 is Ok(6.480741)
// The square root of 42 is Ok(6.480741)

// fn multc<T: Trait1, U: Trait1 + Trait2>(x: T, y: U) {}
// fn multc<T, U>(x: T, y: U) where T: Trait1, U: Trait1 + Trait2 {}

