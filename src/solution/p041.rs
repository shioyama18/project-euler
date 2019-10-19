/// [Pandigital prime](https://projecteuler.net/problem=41)
///
/// solve() returns largest pandigital prime
///
/// # Example
///
/// ```ignore
/// assert_eq!(project_euler::solution::p041::solve(), 7652413);
/// ```
pub fn solve() -> usize {
    use crate::util::prime::Prime;
    let p = Prime::new();
    p.take_while(|&x| x < 10_000_000)
        .filter(|&x| is_pandigital(x))
        .max()
        .unwrap()
}

fn is_pandigital(n: usize) -> bool {
    let n = n.to_string();
    let digits = (1..=n.len()).map(|x| x as u32).collect::<Vec<_>>();

    let mut n = n.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
    n.sort();
    n == digits
}
