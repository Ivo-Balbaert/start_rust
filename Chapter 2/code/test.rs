fn main() {
    // Using local inference, the compiler knows that `elem` has type u64 (machine-dependent)
    let elem = 42;
    // Create an empty vector (a growable array)
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`)
    // Insert `elem` in the vector
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // Try commenting out the `vec.push(elem)` line
    // --> error: unable to infer enough type information about `_`; type annotations required [E0282]
    println!("{:?}", vec);
}
// [42]
