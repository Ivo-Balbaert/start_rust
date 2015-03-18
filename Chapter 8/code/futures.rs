use std::sync::Future;

fn fib(n: u64) -> u64 { // calculates the n-th Fibonacci number
    // long computation that returns a u64
    if n == 0 { return 0 };
    if n == 1 { return 1 };
    fib(n - 1) + fib(n - 2)
}

fn do_something_else() {}

fn main() {
	let mut future_value = Future::spawn( || fib(40) );
	do_something_else();
	println!("fib(40) = {:?}", future_value.get());
}
// fib(40) = 102334155
