extern crate rand;
use rand::Rng;

fn main() {
    println!("Give me 5 random numbers:");
    for _ in 0..5 {
        let rnd = rand::random::<i32>();
        print!("{} / ", rnd);
    }
    println!("");
    println!("Give me 5 positive random numbers smaller than 32:");
    for _ in 0..5 {
        let rnd = (rand::random::<u32>() % 32) + 1;
        print!("{} / ", rnd);
    }
    println!("");

    let mut rng = rand::thread_rng();
    if rng.gen() { // random bool
        println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
    }

    // generate a random number in a range, for example: 1 - 100:
     let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    let tuple = rand::random::<(f64, char)>();
    println!("{:?}", tuple)
}
// Give me 5 random numbers:
// -1786096291 / -312872251 / 959357270 / 1391515785 / -1379700184 / 
// Give me 5 positive random numbers smaller than 32:
// 11 / 15 / 28 / 13 / 23 /
// The secret number is: 16
// (0.279622, '\u{583cf}')
