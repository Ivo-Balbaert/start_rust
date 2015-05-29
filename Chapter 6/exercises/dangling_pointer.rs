struct IntNumber<'a> {
    x: &'a i32,
}

fn main() {
	// n does not live long enough to be assigned to m
    // let m: &u32 = { 
    //      let n = &5u32; // error: borrowed value does not live long enough
    //      n
    // }; 
    // let o = *m;   

    // The following will be rejected, since y has a shorter lifetime than x.
    let mut x = &3;
    {
	    let mut y = 4;
		// x = &y; // error: `y` does not live long enough
    } // y is freed here, but x still lives...

    // 
    let x = 1;                    // -+ x goes into scope
                              //  |
    {                         //  |
    //    let y = &5;           // ---+ y goes into scope
    //    let f = IntNumber { x: y }; // ---+ f goes into scope
    //    x = &f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |
}