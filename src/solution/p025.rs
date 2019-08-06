use crate::util::fib::Fibonacci;
use num_bigint::BigUint;

#[deny(missing_docs)]
/// [1000-digit Fibonacci number](https://projecteuler.net/problem=25)
///
/// The Fibonacci sequence is defined by the recurrence relation:
///
/// Fn = F(n−1) + F(n−2), where F1 = 1 and F2 = 1.
///
/// F(12) = 144 is the first term to contain three digits.
///
/// solve(n) finds the index of the first term in Fibonacci sequence to contain n digits.
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p025::solve(1), 1);
/// assert_eq!(project_euler::solution::p025::solve(3), 12);
/// assert_eq!(project_euler::solution::p025::solve(1000), 4782);
/// ```
///
/// # Panics
/// solve(n) panics if n is 0. Index starts from 1.
pub fn solve(n: usize) -> usize {
    // panic if input is 0
    assert_ne!(n, 0);

    if n == 1 {
        return 1;
    }

    let fib = Fibonacci::<BigUint>::new();
    let n = std::iter::repeat("9")
        .take(n - 1)
        .collect::<String>()
        .parse::<BigUint>()
        .unwrap();

    fib.take_while(|x| *x <= n).count() + 1
}
