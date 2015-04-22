// test module in a different file test.rs:
use super::*;

#[test]
fn cube_of_2_is_8() {
    assert_eq!(cube(2), 8);
}

// other test functions:
// ...