extern crate num_integer;
use num_integer::Integer;

fn square(res: usize) -> usize {
    res * res
}

fn fast_expt_rec_process(value: usize, n: usize) -> usize {
    if n == 0 {
        1
    } else {
        match n.is_even() {
            true => square(fast_expt_rec_process(value, n / 2)),
            false => value * fast_expt_rec_process(value, n - 1),
        }
    }
}
// a b^n - loop invariant
#[allow(dead_code)]
fn fast_expt_rec_procedure(a: usize, b: usize, n: usize) -> usize {
    if n == 1 {
        a * b
    } else {
        match n.is_even() {
            true => fast_expt_rec_procedure(a, b * b, n / 2),
            false => fast_expt_rec_procedure(a * b, b, n - 1),
        }
    }
}

fn fast_expt_iter(mut a: usize, mut b: usize, mut n: usize) -> usize {
    while n != 1 {
        match n.is_even() {
            true => {
                n = n / 2;
                b = b * b;
            }
            false => {
                n = n - 1;
                a = a * b;
            }
        }
    }
    a * b
}

fn fast_expt(value: usize, n: usize) -> usize {
    // fast_expt_rec_procedure(1, value, n)
    fast_expt_iter(1, value, n)
}

fn main() {
    println!("Hello, world!");
    println!("Result Rec:{}", fast_expt_rec_process(2, 32));
    println!("Result Iter:{}", fast_expt(2, 32));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(fast_expt(2, 4), 16);
        assert_eq!(fast_expt(2, 8), 256);
        assert_eq!(fast_expt(2, 3), 8);
        assert_eq!(fast_expt(2, 5), 32);
        assert_eq!(fast_expt(3, 5), 243);
        assert_eq!(fast_expt(2, 10), 1024);
        assert_eq!(fast_expt(2, 11), 2048);
    }
}
