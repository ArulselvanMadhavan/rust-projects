fn fact_iter(n: u64, counter: u64, result: u64) -> u64 {
    if counter > n {
        result
    } else {
        fact_iter(n, counter + 1, result * counter)
    }
}

fn main() {
    println!("Hello world");
    println!("Factorial:{}", fact_iter(8, 1, 1))
}
