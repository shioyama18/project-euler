/// [Digit factorials](https://projecteuler.net/problem=34)
///
/// solve() returns sum of all numbers which are equal to the sum of the factorial of their digits
/// i.e. 1! + 4! + 5! = 1 + 24 + 120 = 145
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p034::solve(), 40730);
/// ```
pub fn solve() -> usize {
    (3..=100_000).filter(|&x| x == factorial_sum(x)).sum()
}

fn factorial_sum(x: usize) -> usize {
    x.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| (2..=x as usize).product::<usize>())
        .sum()
}
