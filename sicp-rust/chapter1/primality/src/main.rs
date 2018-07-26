extern crate num_integer;
extern crate rand;
use num_integer::Integer;
use rand::prelude::*;

fn square(n: usize) -> usize {
    n * n
}

fn exp_mod(a: usize, b: usize, n: usize) -> usize {
    if b == 0 {
        1
    } else {
        match n.is_even() {
            true => square(exp_mod(a, b / 2, n)) % n,
            false => (a * exp_mod(a, b - 1, n)) % n,
        }
    }
}

fn fast_prime_helper(mut rng: ThreadRng, n: usize, times: usize) -> bool {
    if times == 0 {
        true
    } else {
        let a = rng.gen_range(2, n);
        match exp_mod(a, n, n) == a {
            true => fast_prime_helper(rng, n, times - 1),
            false => false, // It is NOT PRIME
        }
    }
}

fn fast_prime(n: usize, times: usize) -> bool {
    fast_prime_helper(thread_rng(), n, times)
}

fn main() {
    println!("Hello, world!{}", fast_prime(3, 5));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_tests() {
        assert_eq!(fast_prime(3), true);
    }
}
