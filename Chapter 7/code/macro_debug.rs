#![feature(trace_macros)] 
#![feature(log_syntax)] 

macro_rules! vector {
    (
        $x:expr,$($y:expr),*
    ) => (
        println!("New argument: {}", $x);
        log_syntax!(vector!($($y),*));
    );
    ( $x:expr ) => (
        println!("New argument: {}", $x);
    )
}

fn main() {
    trace_macros!(true);
    vector!(1, 2, 3, 12);
}