// Fibonacci function

pub fn fib(n: u32) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_0() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn test_fib_1() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_fib_2() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn test_fib_5() {
        assert_eq!(fib(5), 5);
    }

    #[test]
    fn test_fib_10() {
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn test_fib_large() {
        assert_eq!(fib(50), 12586269025); 
    }
}