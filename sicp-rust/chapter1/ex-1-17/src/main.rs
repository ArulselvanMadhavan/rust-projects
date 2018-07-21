extern crate num_integer;
use num_integer::Integer;

fn double(a: usize) -> usize {
    2 * a
}

fn halve(a: usize) -> usize {
    a / 2
}

fn succ_add_iter(a: usize, b: usize, res: usize) -> usize {
    if b == 1 {
        a + res
    } else {
        match b.is_even() {
            true => succ_add_iter(double(a), halve(b), res),
            false => succ_add_iter(a, b - 1, a + res),
        }
    }
}

fn succ_add(a: usize, b: usize) -> usize {
    if b == 0 {
        0
    } else {
        succ_add_iter(a, b, 0)
    }
}

fn main() {
    println!("Hello, world!");
    println!("Test:{}", succ_add(4, 7));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_tests() {
        assert_eq!(succ_add(4, 0), 0);
        assert_eq!(succ_add(4, 19), 76);
    }
}
