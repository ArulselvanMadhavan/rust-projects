extern crate num_integer;
extern crate rand;
use num_integer::Integer;
use rand::prelude::*;
use std::time::Instant;

#[allow(dead_code)]
fn square(n: usize) -> usize {
    n * n
}

fn exp_mod(mut a: usize, mut b: usize, mut n: usize, m: usize) -> usize {
    while n != 1 {
        match n.is_even() {
            true => {
                n = n / 2;
                b = (b * b) % m; // a * (b^2 mod m)
            }
            false => {
                n = n - 1;
                a = (a * b) % m;
            }
        }
    }
    (a * b) % m
}

#[allow(dead_code)]
fn exp_mod_rec(a: usize, b: usize, n: usize) -> usize {
    if b == 0 {
        1
    } else {
        match n.is_even() {
            true => square(exp_mod_rec(a, b / 2, n)) % n,
            false => (a * exp_mod_rec(a, b - 1, n)) % n,
        }
    }
}

// Hate the fact that Rust won't support tail recursion.
fn fast_prime_helper(mut rng: ThreadRng, n: usize, mut times: usize) -> bool {
    let mut result = true;
    while times != 0 {
        let a = rng.gen_range(2, n);
        match exp_mod(1, a, n, n) == a {
            true => {
                times = times - 1;
            }
            false => {
                result = false;
                times = 0; // Exit the loop
            }
        }
    }
    result
}

fn fast_prime(n: usize, times: usize) -> bool {
    fast_prime_helper(thread_rng(), n, times)
}

fn main() {
    let mut res: Vec<_> = (100000..500000)
        .map(|x| {
            let start: Instant = Instant::now();
            let p = fast_prime(x, 10);
            (x, Instant::now().duration_since(start), p)
        })
        .filter(|x| x.2)
        .collect();

    res.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    res.iter().take(10).for_each(|x| println!("{:?}", x));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_tests() {
        assert_eq!(fast_prime(3, 5), true);
        assert_eq!(exp_mod_rec(2, 5, 5), exp_mod(1, 2, 5, 5));
        assert_eq!(exp_mod_rec(3, 7, 7), exp_mod(1, 3, 7, 7));
    }
}
