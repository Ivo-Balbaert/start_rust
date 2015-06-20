use std::io::Write; // in order to be able to use write! on a vector

fn main() {
   let mut vec1 = Vec::new();
   write!(&mut vec1, "test");
   println!("{:?}", vec1);
}
// [116, 101, 115, 116]
