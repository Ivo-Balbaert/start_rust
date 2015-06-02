// error: use of unstable library feature 'asm'
// help: add #![feature(asm)] to the crate attributes to enable
#![feature(asm)]

fn subtract(a: i32, b: i32) -> i32 {
    let sub: i32;
    unsafe {
        asm!("sub $2, $1; mov $1, $0" 
        	 : "=r"(sub) 
        	 : "r"(a), "r"(b)
        	);
    }
    sub
}

fn main() {
    println!("{}", subtract(42, 7)) // 35
}