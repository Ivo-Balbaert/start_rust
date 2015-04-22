use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        println!("Hello from the goblin in the spawned thread!");
    });

    // thread::sleep_ms(50);

    // do other work in the meantime
	let output = handle.join().unwrap(); // ()
	println!("{:?}", output);

	// if no other work has to be done:
	thread::spawn(move || {
        println!("Hello again from the goblin in the spawned thread!");
        // other work done in child thread 
    }).join();
}
// Hello from the goblin in the spawned thread!
// ()
// Hello again from the goblin in the spawned thread!
