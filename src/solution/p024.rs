#[deny(missing_docs)]
/// [Lexicographic permutations](https://projecteuler.net/problem=24)
///
/// A permutation is an ordered arrangement of objects. For example, 3124 is one possible
/// permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically
/// or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
///
/// 012   021   102   120   201   210
///
/// solve(n) finds nth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p024::solve(1_000_000), 2783915460);
/// ```
pub fn solve(mut n: usize) -> usize {
    n -= 1;
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut result = Vec::new();

    while !digits.is_empty() {
        let fact = factorial(digits.len() - 1);
        let quotient = n / fact;
        n %= fact;
        result.push(digits.remove(quotient));
    }

    let mut output = 0;

    for (i, x) in result.into_iter().rev().enumerate() {
        output += x * 10usize.pow(i as u32);
    }

    output
}

/// Returns factorial of n
fn factorial(n: usize) -> usize {
    (2..=n).product()
}
