static mut globvar: i32 = 42;

fn main() {

// error: use of mutable static requires unsafe function or block [E0133]
	unsafe {   // because it is dangerous to change a global variable!
		globvar = 0;
    	println!("My variable global constant: {}", globvar);
	}
	
}
// My variable global constant: 0