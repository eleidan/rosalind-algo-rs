/// This function computes a fibonacci number
///
/// #Examples
///
/// ```
/// use rosalind_algo::fibo::fibonacci;
/// println!("{}", fibonacci(25));
/// ```
pub fn fibonacci(n:usize) -> u64 {
    const MAX_LENGTH: usize = 26;
    let mut calculated_values:[u64; MAX_LENGTH] = [0; MAX_LENGTH];

    calculated_values[1] = 1;

    for i in 2..n+1 {
        calculated_values[i] = calculated_values[i-1] + calculated_values[i-2];
    }

    calculated_values[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
    }
}
