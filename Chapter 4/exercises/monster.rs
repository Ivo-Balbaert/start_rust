struct Monster {
    health: i32,
    damage: i32
}

fn main() {
    let m = Monster { health: 10, damage: 20 };

    println!("{}", m.health);
    println!("{}", m.damage);
}
// 10
// 20