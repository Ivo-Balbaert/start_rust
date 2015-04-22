use std::thread;

fn main() {
    let mut health = 12;
    for i in 2..5 {
        thread::spawn(move || {
            health *= i;
        });
    }
    thread::sleep_ms(2000);
    println!("{}", health); // 12
}