fn main() {
    let items = [1u32, 2, 3, 4];
    let ptr = &items[1] as *const u32;
    println!("{}", unsafe { *ptr });
    println!("{}", unsafe { *ptr.offset(-1) });
    println!("{}", unsafe { *ptr.offset(1) });
    // error: binary operation `+` cannot be applied to type `*const u32` [E0369]
    // println!("{}", unsafe { ptr + 2});
}
// 2
// 1
// 3
