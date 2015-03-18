use std::thread;
use std::sync::mpsc;

static NTHREADS: usize = 7;

fn main() {
    let (tx, rx) = mpsc::channel();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone(); // clone the sender end-point
        thread::spawn(move || {
           thread_tx.send(id).unwrap();
           println!("thread {} done", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    println!("{:?}", ids);
}
// Order is different each time the program is run:
// thread 4 done
// thread 3 done
// thread 1 done
// thread 6 done
// thread 2 done
// thread 0 done
// [Ok(3), Ok(4), Ok(1), Ok(6), Ok(2), Ok(0), Ok(5)]