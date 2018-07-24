extern crate num_integer;
use num_integer::Integer;

fn p_prime(p: &usize, q: &usize) -> usize {
    (p * p) + (q * q)
}

fn q_prime(p: &usize, q: &usize) -> usize {
    (q * q) + (2 * p * q)
}

fn a_prime(a: &usize, b: &usize, p: &usize, q: &usize) -> usize {
    (b * q) + (a * q) + (a * p)
}
fn b_prime(a: &usize, b: &usize, p: &usize, q: &usize) -> usize {
    (b * p) + (a * q)
}
fn fib_iter(mut n: usize) -> usize {
    let mut a = 1usize;
    let mut b = 0usize;
    let mut p = 0usize;
    let mut q = 1usize;
    while n != 0 {
        println!("{}", n);
        match n.is_even() {
            true => {
                let new_p = p_prime(&p, &q);
                q = q_prime(&p, &q);
                p = new_p;
                n = n / 2;
            }
            false => {
                let new_a = a_prime(&a, &b, &p, &q);
                b = b_prime(&a, &b, &p, &q);
                a = new_a;
                n = n - 1;
            }
        }
    }
    b
}

fn fib_rec_proc(a: usize, b: usize, p: usize, q: usize, n: usize) -> usize {
    if n == 0 {
        b
    } else {
        match n.is_even() {
            true => fib_rec_proc(a, b, p_prime(&p, &q), q_prime(&p, &q), n / 2),
            false => fib_rec_proc(
                a_prime(&a, &b, &p, &q),
                b_prime(&a, &b, &p, &q),
                p,
                q,
                n - 1,
            ),
        }
    }
}

fn fib(n: usize) -> usize {
    fib_rec_proc(1, 0, 0, 1, n)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_tests() {
        assert_eq!(fib_iter(4), 3);
        assert_eq!(fib_iter(10), 55);
    }
}
