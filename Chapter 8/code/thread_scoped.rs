use std::thread;

fn main() {
    let guard = thread::scoped(move || {
        println!("Hello from the goblin in the scoped thread!");
    });
// do other work in the meantime
	let output = guard.join();
	println!("{:?}", output);

// if no other work has to be done:
	thread::scoped(move || {
        println!("Hello from the goblin in the scoped thread!");
    }).join();
}
// Hello from the goblin in the scoped thread!
// ()