use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file = BufReader::new(File::open("numbers.txt").unwrap());

    // .lines() eats EOF error, so this is will only panic if something went completely wrong.
    let pairs: Vec<_> = file.lines().map(|line| {
        let line = line.unwrap();
        let line = line.trim();
        let mut words = line.split(" ");
        let left = words.next().expect("Unexpected empty line!");
        let right = words.next().expect("Expected number!");

        (
         left.parse::<u64>().ok().expect("Expected integer in first column!"),
         right.parse::<f64>().ok().expect("Expected float in second column!")
         )
    }).collect();

    println!("{:?}", pairs);
}
// [(120, 345.56), (125, 341.56)]
