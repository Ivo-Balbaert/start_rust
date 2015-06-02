use std::mem;

fn main() {
    let arr = ["Rust", "Go", "Swift"];
    println!("array arr occupies {} bytes", mem::size_of_val(&arr));
    println!("The size of an isize: {} bytes", mem::size_of::<isize>());
}
// array arr occupies 48 bytes
// The size of an isize: 8 bytes