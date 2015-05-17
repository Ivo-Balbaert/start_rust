fn main() {
	// dereferencing a raw pointer:
    let p_raw: *const u32 = &10;
    // error: dereference of unsafe pointer requires unsafe function or block [E0133]
    // let n = *p_raw;

    unsafe {
    	// dereferencing a raw pointer must be done inside unsafe block:
    	let n = *p_raw;
    	println!("{}", n); // 10
    }

    // converting between references and raw pointers:
    let gr: f32 = 1.618;
    let p_imm: *const f32 = &gr as *const f32; // explicit cast
    let mut m: f32 = 3.14;
    let p_mut: *mut f32 = &mut m; // implicit coercion

    unsafe {
        let ref_imm: &f32 = &*p_imm;
        let ref_mut: &mut f32 = &mut *p_mut;
    }
}