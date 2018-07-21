extern crate num_integer;
use num_integer::Integer;
use std::cmp::Ordering;

fn square(res: usize) -> usize {
    res * res
}

fn fast_expt_rec(value: usize, n: usize) -> usize {
    if n == 0 {
        1
    } else {
        match n.is_even() {
            true => square(fast_expt_rec(value, n / 2)),
            false => value * fast_expt_rec(value, n - 1),
        }
    }
}

// Takes more steps to converge.
fn fast_expt_iter(value: usize, n: usize, mut counter: usize, mut result: usize) -> usize {
    loop {
        counter = counter * 2;
        match counter.cmp(&n) {
            Ordering::Less => result = square(result),
            Ordering::Equal => {
                result = square(result);
                break;
            }
            Ordering::Greater => result = result * fast_expt(value, counter - n),
        }
    }
    result
}

fn fast_expt(value: usize, n: usize) -> usize {
    match n {
        0 => 1,
        1 => value,
        _ => fast_expt_iter(value, n, 1, value),
    }
}

fn main() {
    println!("Hello, world!");
    println!("Result Rec:{}", fast_expt_rec(2, 32));
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
