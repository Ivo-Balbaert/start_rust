use std::thread;

static NTHREADS: i32 = 10000;

fn main() {
    for i in 0..NTHREADS {
        let _ = thread::scoped(move || {
            println!("this is thread number {}", i)
        });
    }
}