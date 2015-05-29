use std::io;

fn main() {
    let num:Option<u32> = read_value();

    match num {
            Some(val) => println!("Read: {}", val),
            None => println!("Failed to read number.")
    }
}

fn read_value<T>() -> Option<T> where T: From<T> {
    let mut line = String::new();

    if io::stdin().read_line(&mut line).is_ok() {
// error: the trait `core::str::FromStr` is not implemented for the type `T` [E0277]
            let result = line.trim().parse::<T>();

            return match result {
                    Ok(value) => Some(value),
                    Err(_) => None
            };
    } 

    None
}       