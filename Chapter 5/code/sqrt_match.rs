use std::f32;

fn main() {
    // using Result<T, E>
    let m = sqroot(42.0);
    // let m = sqroot(-5.0);
    
    // match m {
    //  Ok(sq) => println!("The square root of 42 is {}", sq),
    //  Err(str) => println!("{}", str)
    // }

    let res = match m {
        Ok(sq) => sq,
        Err(_) => -1.0
    };
    println!("res is {}", res);
}

fn sqroot(r: f32) -> Result<f32, String> {
    // if r < 0.0 { 
    //  return Err("Number cannot be negative!".to_string()); 
    // }
    assert!(r >= 0.0, "Number cannot be negative!");
    Ok(f32::sqrt(r))
}
// The square root of 42 is 6.480741
// mres is 6.480741
// for m == -5.0: Number cannot be negative!
// res is 6.4807405