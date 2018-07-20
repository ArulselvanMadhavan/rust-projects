fn pascal_rec(n:u64, level:u64, previous:vec<u64>) -> vec<u64> = {
    if level == n {
        previous
    } else {
        previous.iter().fold(Vec::new(),|acc, elem| )
    }
}

fn main() {
    println!("Hello, world!");
}
