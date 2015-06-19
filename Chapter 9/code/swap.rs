use std::mem;

fn main() {
   let mut n = 0;
   let mut m = 1; 
   mem::swap(&mut n, &mut m);
   println!("n: {} m: {}", n, m);
}
// n: 1 m: 0
