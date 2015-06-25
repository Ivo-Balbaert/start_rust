use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::io;

struct Info {
    name: String,
    age: i32,
    rating: i32
}

impl Info {
    fn as_bytes(&self) -> &[u8] {
        let bytes = self.name.as_bytes();
        bytes
    }

    fn format(&self) -> String {
        format!("{};{};{}\n", self.name, self.age, self.rating) 
    }
}

fn main() {
    let path = Path::new("info.txt");
    let display = path.display();

    let file = match write_file(&path) {
        Err(why) => panic!("couldn't write info to file {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };
}

fn write_file(path: &Path) -> Result<File, io::Error> {
    let mut file = try!(File::create(path));
    let info1 = Info { name:"Barak".to_string(), age: 56, rating: 8 };
    let info2 = Info { name:"Vladimir".to_string(), age: 55, rating: 6 };
    try!(file.write(info1.as_bytes()));
    try!(file.write(b"\r\n"));
    try!(file.write(info2.as_bytes()));
    Ok(file)
}
