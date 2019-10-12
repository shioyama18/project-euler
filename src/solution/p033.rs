use num_rational::Ratio;

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
    let mut product = Ratio::new(1, 1);

    for i in 1..10 {
        for j in 1..10 {
            for k in 1..10 {
                if is_curious(i, j, k) && i != j && i != k {
                    product *= Ratio::new(i, k);
                }
            }
        }
    }

    *product.denom()
}

fn is_curious(i: usize, j: usize, k: usize) -> bool {
    Ratio::new(10 * i + j, 10 * j + k) == Ratio::new(i, k)
}
