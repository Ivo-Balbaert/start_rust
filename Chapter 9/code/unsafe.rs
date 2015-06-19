use std::mem;

static mut n: i32 =  42;

fn main() {
	let v: &[u8] = unsafe { 
		mem::transmute("Gandalf") 
	};
	println!("{:?}", v);

    // reading or changing a static mutable variable:
    unsafe {
       println!("{:?}", n );  // 42
       n = 108;
    }
}
// [71, 97, 110, 100, 97, 108, 102]