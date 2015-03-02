fn main() {
	// n does not live long enough to be assigned to m
    // let m: &u32 = { 
    //     let n = &5u32; // error: borrowed value does not live long enough
    //     &*n
    // }; 
    // let o = *m;   
}