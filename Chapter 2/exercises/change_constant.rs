static MAXHEALTH: i32 = 100;
static GAMENAME: &'static str = "Monster Attack";

fn main() {
// 	MAXHEALTH = 99; // error: cannot assign to immutable static item
}
/*change_constant.rs:5:2: 5:16 error: cannot assign to immutable static item
change_constant.rs:5 	MAXHEALTH = 99;
error: aborting due to previous error
[Finished in 0.8s]*/
