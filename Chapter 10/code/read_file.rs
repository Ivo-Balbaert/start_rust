use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display(); 

    // opening a file:
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why))
    };

    // reading a file in one chunk:
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, content),
    }
}
// hello.txt contains:
// "Hello Rust World!" 