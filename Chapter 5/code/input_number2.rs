use std::io;

fn main() {
    println!("Give an integer number bigger than 0:");
    let num = read_u32();

    match num {
        Some(val) => println!("That's the number: {}", val),
        None => println!("Failed to read number.")
    }
}

fn read_u32() -> Option<u32> {
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_ok() {
            let result = buf.trim().parse::<u32>();
            return match result {
                    Ok(value) => Some(value),
                    Err(_) => None  //Failed to parse
            };
    } 
    //Failed to read from stream
    None
}