use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();
    
    thread::spawn(move|| {
         let result = some_expensive_computation();
         tx.send(result).ok().expect("Unable to send message");
    });

    some_other_expensive_computation();
    let result = rx.recv();
    println!("{:?}", result);  // Ok(1)
}

fn some_expensive_computation() -> i32 { 1 }
fn some_other_expensive_computation() { }