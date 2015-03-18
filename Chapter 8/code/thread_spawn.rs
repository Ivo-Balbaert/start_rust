use std::thread;
use std::old_io::timer;
use std::time::Duration;

fn main() {
    thread::spawn(move || {
        println!("Hello from the goblin in the spawned thread!");
    });

    timer::sleep(Duration::milliseconds(50));
}