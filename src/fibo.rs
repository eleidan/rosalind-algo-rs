use num::bigint::BigUint;

/// This function computes Fibonacci number
///
/// #Examples
///
/// ```
/// use rosalind_algo::fibo::fibonacci;
/// assert_eq!(fibonacci(50).to_str_radix(10), "12586269025");
/// println!("{}", fibonacci(50));
/// ```
pub fn fibonacci(n:usize) -> BigUint {
    let mut f0: BigUint = BigUint::new(vec!(0));
    let mut f1: BigUint = BigUint::new(vec!(1));

    for _ in 0..n {
        let f2 = &f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fibonacci(0).to_str_radix(10), "0");
        assert_eq!(fibonacci(1).to_str_radix(10), "1");
        assert_eq!(fibonacci(2).to_str_radix(10), "1");
        assert_eq!(fibonacci(3).to_str_radix(10), "2");
        assert_eq!(fibonacci(4).to_str_radix(10), "3");
        assert_eq!(fibonacci(5).to_str_radix(10), "5");
        assert_eq!(fibonacci(6).to_str_radix(10), "8");
        assert_eq!(fibonacci(7).to_str_radix(10), "13");
        assert_eq!(fibonacci(50).to_str_radix(10), "12586269025");
    }
}
