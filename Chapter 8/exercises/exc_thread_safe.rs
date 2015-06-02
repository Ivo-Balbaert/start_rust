use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
	let data = Arc::new(Mutex::new( vec![7, 13, 42] ));
    (0..3).map(|i| {
        let mutex = data.clone();
        thread::spawn(move || {
            let mut vec = mutex.lock().unwrap();
            vec[i] *= 3;
        }).join()
    }).collect::<Vec<_>>();
    println!("{:?}", *data.lock().unwrap());
}
// [21, 39, 126]
