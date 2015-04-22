use std::thread;
use std::sync::mpsc::channel;
// use std::sync::mpsc::{Sender, Receiver};

fn main() {
	// let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
	let (tx, rx) = channel();
	
	thread::spawn(move|| {
    	// tx.send(10).unwrap();
    	tx.send(10).ok().expect("Unable to send message");
	});

	let res = rx.recv().unwrap(); 
	println!("{:?}", res); // 10
}
// 10