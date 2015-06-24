 fn main() {
    if let Some(arg1) = std::env::args().nth(1) {
        if let Ok(x) = arg1.parse::<i32>() {
            println!("Got it: {}", x);
        } else {
            println!("I wasn't given an integer!");
        }
    } else {
        println!("I wasn't given an argument!");
    } 
}