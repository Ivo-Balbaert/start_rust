struct Block {
    number: i32
}

fn main() {
    let block = Block{ number:1 };
    // ordinary closure:
    let closure = || { println!("n: {:?}", block.number); };
    closure();
    println!("n: {:?}", block.number);

    let block = Block{ number:1 };
    // moving closure:
    // closure takes ownership of the block value
    let closure = move || {println!("n: {:?}", block.number); };
    closure();
    // error: use of moved value: `block.number`
    // println!("n: {:?}", block.number);
}
// n: 1
// n: 1
// n: 1