use game1::Magician;

mod game1 {
    #[derive(Debug)]
    pub struct Magician {
        pub name: String,
        pub age: i32,
        power: i32
    }

    impl Magician {
        // A public constructor
        pub fn new(nm: String, ag: i32, pow: i32) -> Magician {
            Magician { name: nm, age: ag, power: pow} 
        }
    }
}

fn main() {
    // error: field `power` of struct `game1::Magician` is private
    // let mag1 = game1::Magician { name: "Gandalf", age: 725, power: 98};

    let mag1 = Magician::new("Gandalf".to_string(), 725, 98);
    println!("I just made a magician {:?}", mag1);
}
// I just made a magician Magician { name: "Gandalf", age: 725, power: 98 }