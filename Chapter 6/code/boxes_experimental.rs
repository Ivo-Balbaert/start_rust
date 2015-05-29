// #![feature(box_patterns)]

struct Alien {
    planet: String,
    n_tentacles: u32,
}

fn main() {
// error: box expression syntax is experimental; you can call `Box::new` instead.
//    let a = box 5i32;
// mutable reference to a boxed value:
//    let mut a0 = box Alien{ planet: "Mars".to_string(), no_tentacles: 4 };
//    println!("{}", a0.no_tentacles); // 4

// The value can be extracted by the de-structuring pattern: let box m = a; 
// a mechanism which is also called unboxing a into m.
// extract the value:
//    let box m = a;
//    println!("{}", m);

}
   