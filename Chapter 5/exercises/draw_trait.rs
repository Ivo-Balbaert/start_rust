trait Draw {
   fn draw(&self);
}

struct S1 {
   data_s1: i32
}
struct S2 {
   data_s2: f64
}

impl Draw for S1 {
   fn draw(&self) {
       println!("{}", self.data_s1);
   }
}

impl Draw for S2 {
   fn draw(&self) {
	  println!("{}", self.data_s2);
   }
}

fn draw_object<T: Draw>(object: T) {
   println!("Going to draw an object:");
   object.draw();
   println!("Look how beautiful!");
}


fn main() {
   let s1 = S1 { data_s1: 42 };
   let s2 = S2 { data_s2: 42.0 };
   draw_object(s1);    // OK, S1 implements Draw.
   draw_object(s2);    // OK, S2 implements Draw.
   // draw_object(42.0); // error: the trait `Draw` is not implemented for the type `_`
}
// Going to draw an object:
// 42
// Look how beautiful!
// Going to draw an object:
// 42
// Look how beautiful!