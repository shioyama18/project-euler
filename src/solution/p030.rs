#[deny(missing_docs)]
/// [Digit fifth powers](https://projecteuler.net/problem=30)
///
/// solve(n) returns sum of all the numbers that can be written as sum of nth
/// powers of their digits.
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p030::solve(4), 19316);
/// assert_eq!(project_euler::solution::p030::solve(5), 443839);
/// ```
pub fn solve(n: u32) -> u32 {
    let (_, limit) = (1..)
        .map(|x| 9u32.pow(n as u32) * x)
        .zip((1..).map(|x| 10u32.pow(x)))
        .skip_while(|(a, b)| a > b)
        .take(1)
        .next()
        .unwrap();

    (2..=limit).filter(|&x| x == power(x, n)).sum()
}

fn power(x: u32, n: u32) -> u32 {
    x.to_string()
        .chars()
        .map(|i| i.to_digit(10).unwrap().pow(n))
        .sum()
}
