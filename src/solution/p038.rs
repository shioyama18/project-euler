/// [Pandigital multiples](https://projecteuler.net/problem=38)
///
/// solve() returns largest 1 to 0 pandigital 9-digit number that can be formed as the concatenated products
/// of an integer with 1, 2, .., n where n > 1
/// i.e. 192 * 1 ++ 192 * 2 ++ 192 * 3 = 192384576
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p038::solve(), 932718654);
/// ```
pub fn solve() -> usize {
    (9234..=9487)
        .map(|x| 100_002 * x)
        .filter(|&x| is_pandigital(x))
        .max()
        .unwrap()
}

fn is_pandigital(n: usize) -> bool {
    let mut n = n.to_string().chars().collect::<Vec<_>>();
    n.sort();
    n.into_iter().collect::<String>() == "123456789"
}
