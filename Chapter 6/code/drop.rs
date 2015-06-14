struct Block {
    number: i32
}

impl Drop for Block {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn print_block(block: Block) {
    println!("In function print_block");
}

fn main() {
    let block = Block{ number: 1 };
    // move of value block:
    print_block(block);
    println!("Back in main!");
}
// In function print_block
// Dropping!
// Back in main!