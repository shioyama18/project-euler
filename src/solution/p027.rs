#[deny(missing_docs)]
/// [Quadratic primes](https://projecteuler.net/problem=27)
///
/// solve() finds products of the coefficient a and b for the
/// quadratic expression that produces maximum number of primes for
/// consecutive values of n starting with n = 0.
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p027::solve(), -59231);
/// ```
pub fn solve() -> isize {
    let m = (1..)
        .filter(|x| x * x - x + 41 > 1000)
        .take(1)
        .next()
        .unwrap();
    let a = m - 1;
    -(2 * a - 1) * (a * a - a + 41)
}
