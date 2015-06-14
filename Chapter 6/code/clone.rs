struct Block {
    number: Box<i32>
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Block{ number: self.number.clone() }
    }
}

fn print_block(block: Block) {
    println!("{:p}: {:?}", block.number, block.number);
}

fn main() {
    let block = Block{ number: Box::new(1) };
    println!("{:p}: {:?}", block.number, block.number);

    print_block(block.clone());
}
// 0x2424008: 1
// 0x2424028: 1