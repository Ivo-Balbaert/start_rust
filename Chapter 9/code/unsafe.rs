use std::mem;

fn main() {
	let v: &[u8] = unsafe { 
		mem::transmute("Gandalf") 
	};
	println!("{:?}", v);
}
// [71, 97, 110, 100, 97, 108, 102]