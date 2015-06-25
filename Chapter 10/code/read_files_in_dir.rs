use std::env;
use std::fs;
use std::error::Error;

fn main() {
    show_dir().unwrap();
}

fn show_dir() -> Result<(), Box<Error>> {
    let here = try!(env::current_dir());
    println!("Contents in: {}", here.display());
    for entry in try!(fs::read_dir(&here)) {
        let path = try!(entry).path();
        let md = try!(fs::metadata(&path));
        println!("  {} ({} bytes)", path.display(), md.len());
    }
    Ok(())
}
// Contents in: F:\Rust\Rust book\The Rust Programming Language\Chapter 10 - Working with files\code
// F:\Rust\Rust book\The Rust Programming Language\Chapter 10 - Working with files\code\read_file.rs (710 bytes)
// F:\Rust\Rust book\The Rust Programming Language\Chapter 10 - Working with files\code\read_files_in_dir.exe (2382143 bytes)
// F:\Rust\Rust book\The Rust Programming Language\Chapter 10 - Working with files\code\read_files_in_dir.rs (449 bytes)
// F:\Rust\Rust book\The Rust Programming Language\Chapter 10 - Working with files\code\read_file_try.exe (2393158 bytes)



