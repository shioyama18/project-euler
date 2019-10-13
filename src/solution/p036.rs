/// [Double-base palindromes](https://projecteuler.net/problem=36)
///
/// solve(n) returns sum of all numbers below n that are palindromic in base 10 and base 2.
/// i.e. 585 == 1001001001
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p036::solve(1_000_000), 872187);
/// ```
pub fn solve(n: usize) -> usize {
    (1..n)
        .filter(|&x| is_palindrome_base10(x) && is_palindrome_base2(x))
        .sum()
}

fn is_palindrome_base10(n: usize) -> bool {
    let n_rev = n
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .fold(0, |acc, x| 10 * acc + x) as usize;

    n == n_rev
}

fn is_palindrome_base2(n: usize) -> bool {
    let n = format!("{:b}", n);
    let n_rev = n.chars().rev().collect::<String>();

    n == n_rev
}
