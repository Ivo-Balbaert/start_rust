use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::io;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display(); 

    let content = match read_file(path) {
         Err(why) => panic!("error reading {}: {}", display, Error::description(&why)),
         Ok(content) => content
    };

    println!("{}", content);
}

fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut file = try!(File::open(path));
    let mut buf = String::new();
    try!(file.read_to_string(&mut buf));
    Ok(buf)
}
// "Hello Rust World!" 
