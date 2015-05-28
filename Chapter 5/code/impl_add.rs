use std::ops::Add;

#[derive(Debug)]
struct AType {
    value: i32,
}

impl AType {
    fn new(value: i32) -> AType {
        AType { value: value }
    }
}

impl Add for AType {
    type Output = AType;

    fn add(self, other: AType) -> AType {
        AType { value: self.value + other.value }
    }
}


fn main() {
    let at1 = AType{ value: 7 };
    let at2 = AType{ value: 42 };
    let at3 = at1.add(at2);
    println!("{:?}", at3);
}
