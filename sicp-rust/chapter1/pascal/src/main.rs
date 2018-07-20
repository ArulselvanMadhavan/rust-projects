fn pascal_rec(n: usize, level: usize, previous: Vec<usize>) -> Vec<usize> {
    if level == n {
        previous
    } else {
        let mut next = Vec::with_capacity(level);
        previous.into_iter().fold(0usize, |p, elem| {
            next.push(p + elem);
            elem
        });
        next.push(1);
        pascal_rec(n, level + 1, next)
    }
}

fn pascal(n: usize) -> Vec<usize> {
    pascal_rec(n, 1, vec![1])
}

fn main() {
    println!("Hello, world!");
    println!("Pascal {:?}", pascal(5));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(pascal(5), vec![1, 4, 6, 4, 1]);
    }
}
