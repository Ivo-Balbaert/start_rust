use std::old_io::Timer;
use std::old_io::timer;
use std::time::duration::Duration;
use std::iter;
use std::sync::mpsc;

fn main() {
    let moment = Duration::milliseconds(2500);
    let mut timer = Timer::new().unwrap();
    let oneshot = timer.oneshot(moment);
    let _ = oneshot.recv(); // blocks for moment of time

    let clock = timer.periodic(moment);
    println!("Ticking every 2,5 seconds");
    for i in iter::range_step(10i32, 0, -1) {
        let _ = clock.recv();
        println!("tick {}", i);
    }
    let _ = clock.recv();
    println!("End of battery time!");
}
// Ticking every 2,5 seconds
// tick 10
// tick 9
// tick 8
// tick 7
// tick 6
// tick 5
// tick 4
// tick 3
// tick 2
// tick 1
// End of battery time!