fn ackermann(x: u64, y: u64) -> u64 {
    match (x, y) {
        (_, 0) => 0,
        (0, _) => 2 * y,
        (_, 1) => 2,
        (_, _) => ackermann(x - 1, ackermann(x, y - 1)),
    }
}

fn main() {
    println!("Hello, world!");
    println!("Ackermann:{}", ackermann(1, 10));
    println!("Ackermann:{}", ackermann(2, 1));
    println!("Ackermann:{}", ackermann(2, 2));
    println!("Ackermann:{}", ackermann(2, 3));
    println!("Ackermann:{}", ackermann(2, 4));
    println!("Ackermann:{}", ackermann(3, 3));
}

// Ackermann
// A = { 0 ; y = 0, 2y; x = 0; 2; y =1; otherwise A (x - 1) (A x (y - 1))
// f(n) = A(0, n) = 2n
// g(n) = A(1, n) = A(0,(A (1, n-1))) = 2(A(1, n-1)) = 2^2(A(1, n-2)) = 2^n-2(A(1,1)) = 2^n-2 * 2 = 2^n-1
// h(n) = A(2, n) = A(1,A(2, n-1)) = A(0,A(1,A(2,n-1)) - 1) = 2(A(1, A(2, n - 1)) - 1) = 2(2^(A(2,n-1)-1)) = 2^(A(2, n-1))
