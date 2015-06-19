fn main() {
    let current_col = column!();
    println!("defined on column: {}", current_col);

    let current_line = line!();
    println!("defined on line: {}", current_line);
    
    let this_file = file!();
    println!("defined in file: {}", this_file);

    not_ready();
}

fn not_ready() {
    unimplemented!();
}
// defined on column: 22
// thread 'defined on line: 5
// defined in file: F:\Rust\Rust book\The Rust Programming Language\Chapter 7 - Organizing code and macros\code\builtin_macros.rs
// <main>' panicked at 'not yet implemented', F:\Rust\Rust book\The Rust Programming Language\Chapter 7 - Organizing code and macros\code\builtin_macros.rs:15