use std::thread;
use std::old_io::timer;
use std::time::duration::Duration;

fn main() {
    let mut health = 12;
    for i in 2..5 {
        thread::scoped(move || {
            health *= i;
        });
    }
    timer::sleep(Duration::milliseconds(2000));
    println!("{}", health); // 12
}