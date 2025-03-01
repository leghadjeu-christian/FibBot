use num_bigint::{BigUint, ToBigUint};

/// Computes the nth Fibonacci number using BigInt.
pub fn fib(n: u32) -> BigUint {
    let mut f0 = 0.to_biguint().unwrap();
    let mut f1 = 1.to_biguint().unwrap();

    if n == 0 {
        f0
    } else if n == 1 {
        f1
    } else {
        for _ in 2..=n {
            let f2 = &f0 + &f1;
            f0 = f1;
            f1 = f2;
        }

        f1
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            fib(100),
            BigUint::from_str("354224848179261915075").unwrap()
        );
        assert_eq!(
            fib(22),
            BigUint::from_str("17711").unwrap()
        );
    }
}
