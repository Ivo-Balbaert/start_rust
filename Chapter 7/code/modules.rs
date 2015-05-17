use game1::func2;
use game1::func2 as gf2;
// use game1::{func2, func3};
// use game1::*;
use game1::subgame1::subfunc1 as sf1;

mod game1 {
    // all of the module's code items go in here
    fn func1() { // private function
    	println!("Am I visible?");
    }

    pub fn func2() {
    	println!("You called func2 in game1!");
    }

    pub fn func3() {
    	println!("You called func2 in game1!");
    }

    pub mod subgame1 {
    	pub fn subfunc1() {
    		println!("You called subfunc1 in subgame1!");
    	}
    }

    pub struct Magician {
        pub name: String,
        pub age: i32,
        power: i32
    }
}

fn main() {
	// game1::func1(); // error: function `func1` is private
    game1::func2(); // works without the use import

	// calling a nested module:
    game1::subgame1::subfunc1();

	// importing a function or module with use:
    func2();
    gf2();
    sf1();

   // error: field `power` of struct `game1::Magician` is private
   // let mag1 = game1::Magician { name: "Gandalf".to_string(), age: 725, power: 98};
}
   
// You called func2 in game1!
// You called subfunc1 in subgame1!
// You called func2 in game1!
// You called func2 in game1!
// You called subfunc1 in subgame1!