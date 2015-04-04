use std::iter;

fn main() {
    for i in iter::range_step(5i32, 0, -1) {
    	print!("{} - ", i);
    }
}
// 5 - 4 - 3 - 2 - 1 -