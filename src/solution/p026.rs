use std::collections::VecDeque;

#[deny(missing_docs)]
/// [Reciprocal cycles](https://projecteuler.net/problem=26)
///
/// A unit fraction contains 1 in the numerator.
/// The decimal representation of the unit fractions with denominators 2 to 10 are given:
///
/// 1/2 = 0.5
/// 1/3 = 0.(3)
/// 1/4 = 0.25
/// 1/5 = 0.2
/// 1/6 = 0.1(6)
/// 1/7 = 0.(142857)
/// 1/8 = 0.125
/// 1/9 = 0.(1)
/// 1/10 = 0.1
///
/// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle.
/// It can be seen that 1/7 has a 6-digit recurring cycle.
///
/// solve(n) finds vlue of d < n for which 1/d contains the longest recurring cylce
/// in its decimal fraction part.
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p026::solve(10), 7);
/// assert_eq!(project_euler::solution::p026::solve(1000), 983);
/// ```
///
/// # Panics
/// solve(n) panics if the input is less than or equal to 1
pub fn solve(n: usize) -> usize {
    (1..n)
        .map(recurring_cycle)
        .max()
        .expect("Input needs to be larger than 1")
}

/// returns the length of reciprocal cycle of divisor
fn recurring_cycle(divisor: usize) -> usize {
    let mut rs = VecDeque::new();
    remainders(divisor, 10, &mut rs) + 1
}

/// helper function to determine the length of reciprocal cycle
fn remainders(divisor: usize, r: usize, rs: &mut VecDeque<usize>) -> usize {
    // if it is evenly divisible, length of cycle is 0
    if r == 0 {
        return 0;
    }

    let r = r % divisor;
    match rs.iter().position(|&i| i == r) {
        Some(x) => x + 1,
        None => {
            rs.push_front(r);
            remainders(divisor, 10 * r, rs)
        }
    }
}
