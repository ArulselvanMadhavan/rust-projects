fn ackermann(x: u64, y: u64) -> u64 {
    match (x, y) {
        (_, 0) => 0,
        (0, _) => 2 * y,
        (_, 1) => 2,
        (_, _) => ackermann(x - 1, ackermann(x, y - 1)),
    }
}

fn main() {
    println!("Hello, world!");
    println!("Ackermann:{}", ackermann(1, 10));
    println!("Ackermann:{}", ackermann(2, 4));
    println!("Ackermann:{}", ackermann(3, 3));
}
