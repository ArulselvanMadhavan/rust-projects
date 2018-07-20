fn fact_iter(n: u64) -> u64 {
    let mut result = 1;
    let mut counter = 1;
    while counter < n {
        result = result * counter;
        counter = counter + 1;
    }
    result
}

// No tail recursion support
#[allow(dead_code)]
fn fact_rec(n: u64, counter: u64, result: u64) -> u64 {
    if counter > n {
        result
    } else {
        fact_rec(n, counter + 1, result * counter)
    }
}

fn main() {
    println!("Hello world");
    println!("Factorial:{}", fact_iter(20)) // What to do for integers larger than u64?
}
