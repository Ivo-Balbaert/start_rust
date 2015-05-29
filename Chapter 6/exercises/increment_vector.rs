fn main() {
   let p = vec![1, 2, 3];
   let q = increment(p);
   print!("new vector: ");
   for x in &q {
      print!("{} ", x);
   }
   println!("");

   let mut p = vec![1, 2, 3];
   increment_mut(&mut p);
   print!("change in place: ");
   for x in &p {
      print!("{} ", x);
   }

}

fn increment_mut(v: &mut Vec<i32>) {
	for i in 0..v.len() {
		v[i] += 1;
	}
}

fn increment(mut v: Vec<i32>) -> Vec<i32> {
	for i in 0..v.len() {
		v[i] += 1;
	}
	v
}
// new vector: 2 3 4 
// change in place: 2 3 4