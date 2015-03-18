use std::thread;
use std::sync::mpsc;

fn main() {
	let (tx, rx) = mpsc::channel();
    
   thread::spawn(move|| {
        let result = some_expensive_computation();
        tx.send(result);
    });

    some_other_expensive_computation();
    let result = rx.recv();
    println!("{:?}", result);  // Ok(1)
}

fn some_expensive_computation() -> i32 { 1 }
fn some_other_expensive_computation() { }