fn fib(n: u16) -> u32 {
    if n < 2 {
        return n.into();
    }
    fib(n-2) + fib(n-1)
}

#[test]
fn fib_test() {
    assert_eq!(fib(20), 6765);
}
