macro_rules! mac2 {
    ($arg:expr) => (3 * $arg);
}

macro_rules! mac3 {
    ($arg:ident) => (let $arg = 42);
}

macro_rules! mac4 {
    ($arg:expr) => ( { 
    	print!("start - ");
    	print!("{} - ", $arg);
    	print!("end");
    });
}

macro_rules! says {
    ($x:expr) => {
        println!("He/She sais: '{}'", $x);
    };
    ($x:expr, $y:expr) => {
        println!("He/She sais: '{}' to {}", $x, $y);
    };
}


fn main() {
    println!("{}", mac2!(5)); // 15
    println!("{}", mac2!(2 + 3)); // 15
    mac3!(x); // expands into let x = 42;
    println!("{}", x); // 42
    mac4!("Where am I?"); // start - Where am I? - end
    println!("");
    says!("Hi");
    says!("Hi", "Jim");
}
// 15
// 15
// 42
// start - Where am I? - end
// He/She sais: 'Hi'
// He/She sais: 'Hi' to Jim