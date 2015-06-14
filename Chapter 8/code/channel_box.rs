use std::thread;
use std::sync::mpsc;

trait Message : Send {
    fn print(&self);
}

struct Msg1 {
    value: i32
}

impl Message for Msg1 {
    fn print(&self) {
        println!("value: {:?}", self.value);
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<Box<Message>>();
    
    let handle = thread::spawn(move|| {
        let msg = rx.recv().unwrap();
        msg.print();
    });
    
    let msg = Box::new(Msg1{ value:1 });
    tx.send(msg).unwrap();
    
    handle.join().ok();
}
// value: 1