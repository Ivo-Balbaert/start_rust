use std::sync::mpsc;
use std::sync::mpsc::Receiver;

type TokenType = i32;

struct Msg {
    typ: TokenType,
    val: String,
}

fn make_chan() -> Receiver<Msg> {
    let (tx, rx) = mpsc::sync_channel(1); // buffer size 1
    let _ = tx.send(Msg {typ: 42, val: "Rust is cool".to_string()});
    rx
}

fn main() {
    let rx = make_chan();
    if let Some(msg) = rx.recv().ok() {
        println!("received message of type {} and val {}", msg.typ, msg.val);
    };
}
// received message of type 42 and val Rust is cool