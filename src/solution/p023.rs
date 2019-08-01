#[deny(missing_docs)]

/// [Non-abundant sums](https://projecteuler.net/problem=23)
///
/// A perfect number is a number for which the sum of its proper divisors is exactly equal
/// to the number. For example, the sum of the proper divisors of 28 would be
/// 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
///
/// A number n is called deficient if the sum of its proper divisors is less than n and
/// it is called abundant if this sum exceeds n.
///
/// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that
/// can be written as the sum of two abundant numbers is 24. By mathematical analysis,
/// it can be shown that all integers greater than 28123 can be written as the sum of
/// two abundant numbers. However, this upper limit cannot be reduced any further by
/// analysis even though it is known that the greatest number that cannot be expressed as
/// the sum of two abundant numbers is less than this limit.
///
/// solve() finds the sum of all the positive integers which cannot be written as the
/// sum of two abundant numbers.
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p023::solve(28123), 4179871);
/// ```
use crate::util::prime::Prime;

pub fn solve(n: usize) -> usize {
    let abunds = (2..=n)
        .filter(|&i| Prime::sum_proper_divisor(i) > i)
        .collect::<Vec<_>>();

    let mut sum_of_abunds = 0;
    let mut is_sum_of_abunds = vec![false; n + 1];

    for (i, &a) in abunds.iter().enumerate() {
        for &b in &abunds[i..] {
            let sum = a + b;

            if sum > n {
                break;
            }

            if !is_sum_of_abunds[sum] {
                sum_of_abunds += sum;
                is_sum_of_abunds[sum] = true;
            }
        }
    }

    let sum_all = n * (n + 1) / 2;
    sum_all - sum_of_abunds
}
