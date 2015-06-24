fn main() {
    let mut vec1: Vec<i32> = (0..10).collect();
    vec1.retain(|&x| !is_odd(x));
    println!("{:?}", vec1)
}

fn is_odd(n: i32) -> bool { n % 2 != 0 }
// [0, 2, 4, 6, 8]