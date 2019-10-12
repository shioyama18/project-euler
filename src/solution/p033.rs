/// [Digit cancelling fractions](https://projecteuler.net/problem=33)
///
/// solve() returns product of four denominators that satisfies digit cancelling fractions
/// composed of two-digit numerator and two-digit denominator.
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p033::solve(), 100);
/// ```
pub fn solve() -> usize {
    let mut numerator = 1;
    let mut denominator = 1;

    for i in 1..10 {
        for j in 1..10 {
            for k in 1..10 {
                if is_curious(i, j, k) && i != j && i != k {
                    numerator *= i;
                    denominator *= k;
                }
            }
        }
    }

    denominator / gcd(numerator, denominator)
}

fn is_curious(i: usize, j: usize, k: usize) -> bool {
    let numerator = 10 * i + j;
    let denominator = 10 * j + k;

    numerator / gcd(numerator, denominator) == i / gcd(i, k)
        && denominator / gcd(numerator, denominator) == k / gcd(i, k)
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}
